use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{
    BoxError, extract::Host, handler::HandlerWithoutStateExt, http::Uri, response::Redirect,
};
use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration},
};
use axum_server::tls_rustls::RustlsConfig;
use marzban_api::{client::MarzbanAPIClient, models::auth::BodyAdminTokenApiAdminTokenPost};
use reqwest::StatusCode;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tera::Tera;
use tracing::{debug, info, warn};

use crate::{
    config::{GLOBAL_CONFIG, HttpOrHttps},
    routes::create_router,
    sessions::Backend,
};
use argon2::PasswordHasher;

/// Connect to the database from the config file
async fn init_db() -> Result<MySqlPool> {
    // Initialize the database connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(150)
        .connect(&GLOBAL_CONFIG.database_url)
        .await
        .context("Failed to connect to database")?;
    Ok(pool)
}

/// Migrate the database to the latest version
async fn migrate_db(pool: &MySqlPool) -> Result<()> {
    // Run the migrations
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Failed to migrate database")?;
    Ok(())
}

/// Create initial admin user if no users exist
async fn create_initial_admin_user(pool: &MySqlPool) -> Result<()> {
    // Check if there are any users in the database
    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(pool)
        .await
        .context("Failed to fetch user count")?
        .count;

    if user_count == 0 {
        // Create default admin user
        info!("No users found in database. Creating default admin user");

        let default_admin_username = "admin";
        let default_admin_password = "admin";

        // Hash the password
        let password_hash = tokio::task::spawn_blocking(move || {
            let argon2 = argon2::Argon2::default();
            let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
            argon2
                .hash_password(default_admin_password.as_bytes(), &salt)
                .expect("Failed to hash password")
                .to_string()
        })
        .await
        .context("Failed to hash password in thread")?;

        // Register the user
        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, is_admin, created_at, updated_at)
            VALUES (?, ?, ?, NOW(), NOW())
            "#,
            default_admin_username,
            password_hash,
            true
        )
        .execute(pool)
        .await
        .context("Failed to insert user into db")?;

        warn!("Default admin user created. Please change the password immediately");
        warn!("Username: {}", default_admin_username);
        warn!("Password: {}", default_admin_password);
    } else {
        debug!("Users found in database. Doing nothing...");
    }

    Ok(())
}

/// Setup and authenticate with Marzban Panel
async fn setup_marzban_client() -> Result<MarzbanAPIClient> {
    let marzban_client = MarzbanAPIClient::new(&GLOBAL_CONFIG.marzban_panel_url);

    // Authenticate with Marzban Panel
    marzban_client
        .authenticate(&BodyAdminTokenApiAdminTokenPost {
            grant_type: Some("password".to_string()),
            username: GLOBAL_CONFIG.marzban_panel_username.clone(),
            password: GLOBAL_CONFIG.marzban_panel_password.clone(),
            scope: "".to_string(),
            client_id: None,
            client_secret: None,
        })
        .await
        .context("Failed to authenticate with Marzban Panel")?;

    Ok(marzban_client)
}

/// Setup auth layer
async fn setup_auth_layer(pool: &MySqlPool) -> Result<AuthManagerLayer<Backend, MemoryStore>> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    let backend = Backend::new(pool.clone());
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> =
        AuthManagerLayerBuilder::new(backend, session_layer).build();

    Ok(auth_layer)
}

/// Tera template engine setup
async fn setup_tera() -> Result<Tera> {
    let tera = Tera::new(&(GLOBAL_CONFIG.templates_dir.clone() + "/*.html"))
        .context("Failed to load Tera template engine")?;
    Ok(tera)
}

/// Redirect HTTP to HTTPS
async fn redirect_http_to_https() {
    fn make_https(host: String, uri: Uri) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(
            &GLOBAL_CONFIG.http_port.to_string(),
            &GLOBAL_CONFIG.https_port.to_string(),
        );
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = (GLOBAL_CONFIG.ip_addr.clone() + GLOBAL_CONFIG.http_port.to_string().as_str())
        .parse::<SocketAddr>()
        .unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(
        "Redirecting http://{} to https counterpart",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}

/// # Entry Point
///
/// This serves as the main entry point for the backend.
pub async fn init() -> Result<()> {
    // Database setup
    let pool = init_db().await?;

    // Migrate database
    migrate_db(&pool)
        .await
        .context("Failed to migrate database")?;

    // Create initial admin user
    create_initial_admin_user(&pool)
        .await
        .context("Failed to create initial admin user")?;

    // Marzban Panel Client setup
    let marzban_client = setup_marzban_client()
        .await
        .context("Failed to setup Marzban Panel client")?;

    // Stripe setup
    let stripe_client = stripe::Client::new(GLOBAL_CONFIG.stripe_secret_key.clone());

    // Auth service.
    let auth_layer = setup_auth_layer(&pool)
        .await
        .context("Failed to setup auth layer")?;

    // Tera template engine setup
    let tera = setup_tera()
        .await
        .context("Failed to setup Tera template engine")?;

    // Create router
    let app = create_router(auth_layer, pool, marzban_client, stripe_client, tera);

    info!("Router setup");

    // Start the server
    match GLOBAL_CONFIG.http_or_https {
        HttpOrHttps::Http => {
            debug!("We are listening on HTTP only");
            let addr = format!(
                "{}:{}",
                GLOBAL_CONFIG.ip_addr.clone(),
                GLOBAL_CONFIG.http_port
            )
            .parse::<SocketAddr>()
            .unwrap();
            info!("listening on http://{}", addr);

            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        HttpOrHttps::Https => {
            debug!("We are listening on HTTPS");
            let config = RustlsConfig::from_pem_file(
                GLOBAL_CONFIG.https_cert_path.clone(),
                GLOBAL_CONFIG.https_key_path.clone(),
            )
            .await
            .expect("Failed to load HTTPS config. Please check your certs/keys and their paths");

            let addr = format!(
                "{}:{}",
                GLOBAL_CONFIG.ip_addr.clone(),
                GLOBAL_CONFIG.https_port
            )
            .parse::<SocketAddr>()
            .unwrap();
            info!("listening on https://{}", addr);

            if GLOBAL_CONFIG.redirect_to_https {
                debug!("Redirecting HTTP to HTTPS");
                tokio::spawn(redirect_http_to_https());
            } else {
                debug!("Not redirecting HTTP to HTTPS");
            }

            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    };

    Ok(())
}

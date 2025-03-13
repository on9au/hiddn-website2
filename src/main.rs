use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use argon2::PasswordHasher;
use axum::{
    BoxError, extract::Host, handler::HandlerWithoutStateExt, http::Uri, response::Redirect,
};
use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration},
};
use axum_server::tls_rustls::RustlsConfig;
use config::{GLOBAL_CONFIG, HttpOrHttps};
use marzban_api::{client::MarzbanAPIClient, models::auth::BodyAdminTokenApiAdminTokenPost};
use payloads::AnnouncementPayload;
use reqwest::StatusCode;
use routes::create_router;
use sessions::Backend;
use sqlx::mysql::MySqlPoolOptions;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use utils::{load_announcements, load_docs};

mod config;
mod handlers;
mod handlers_admin;
mod payloads;
mod routes;
mod sessions;
mod utils;

type SharedDocs = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

#[tokio::main]
async fn main() {
    // Load dotenv file
    dotenvy::dotenv().ok();

    // Logging/Tracing setup
    tracing_subscriber::fmt::init();

    debug!("Connecting to database");

    // Database setup
    let pool = MySqlPoolOptions::new()
        .max_connections(150)
        .connect(&GLOBAL_CONFIG.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connected to database");

    debug!("Migrating database");

    // Migrate database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    info!("Migrated database");

    debug!("Checking for users in database");

    // Check for users in database
    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user count")
        .count;

    if user_count == 0 {
        // Create default admin user,, prevent no admin user.
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
        .expect("Failed to hash password in thread");

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
        .execute(&pool)
        .await
        .expect("Failed to insert user into db");

        warn!("Default admin user created. Please change the password immediately");
        warn!("Username: {}", default_admin_username);
        warn!("Password: {}", default_admin_password);
    } else {
        debug!("Users found in database. Doing nothing...");
    }

    debug!("Authenticating with Marzban Panel");

    // Marzban Panel Client setup
    let marzban_client = MarzbanAPIClient::new(&GLOBAL_CONFIG.marzban_panel_url);

    // Marzban Panel Authentication setup
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
        .expect("Failed to authenticate with Marzban Panel");

    info!("Authenticated with Marzban Panel");

    debug!("Setting up Stripe client");

    // Stripe setup
    let stripe_client = stripe::Client::new(GLOBAL_CONFIG.stripe_secret_key.clone());

    info!("Stripe client setup");

    debug!("Setting up session layer");

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    info!("Session layer setup");

    debug!("Setting up auth layer");

    // Auth service.
    let backend = Backend::new(pool.clone());
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> =
        AuthManagerLayerBuilder::new(backend, session_layer).build();

    info!("Auth layer setup");

    debug!("Loading documentation and announcements");

    // Load documentation and announcements
    let docs: Arc<RwLock<HashMap<String, HashMap<String, String>>>> =
        Arc::new(RwLock::new(load_docs().await));
    let announcements: Arc<RwLock<Vec<AnnouncementPayload>>> =
        Arc::new(RwLock::new(load_announcements().await));

    info!("Loaded documentation and announcements");

    debug!("Setting up router");

    // Create router
    let app = create_router(
        docs,
        announcements,
        auth_layer,
        pool,
        marzban_client,
        stripe_client,
    );

    info!("Router setup");

    // let listener = TcpListener::bind(GLOBAL_CONFIG.http_socket_addr.clone())
    //     .await
    //     .unwrap();
    // info!("listening on {}", listener.local_addr().unwrap());

    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();

    // Begin listening on specified sockets

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
}

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

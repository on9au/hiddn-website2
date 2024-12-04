use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    extract::Host, handler::HandlerWithoutStateExt, http::Uri, response::Redirect, BoxError,
};
use axum_login::{
    tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder,
};
use axum_server::tls_rustls::RustlsConfig;
use config::{HttpOrHttps, GLOBAL_CONFIG};
use payloads::AnnouncementPayload;
use reqwest::StatusCode;
use routes::create_router;
use sessions::Backend;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::sync::RwLock;
use tracing::info;
use utils::{load_announcements, load_docs};

mod config;
mod handlers;
mod handlers_admin;
mod payloads;
mod routes;
mod sessions;
mod ssr;
mod utils;

type SharedDocs = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

#[tokio::main]
async fn main() {
    // Load dotenv file
    dotenvy::dotenv().ok();

    // Logging/Tracing setup
    tracing_subscriber::fmt::init();

    // Database setup
    let pool = MySqlPoolOptions::new()
        .max_connections(150)
        .connect(&GLOBAL_CONFIG.database_url)
        .await
        .expect("Failed to connect to database");

    // Migrate database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    // Auth service.
    let backend = Backend::default();
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> =
        AuthManagerLayerBuilder::new(backend, session_layer).build();

    // Load documentation and announcements
    let docs: Arc<RwLock<HashMap<String, HashMap<String, String>>>> =
        Arc::new(RwLock::new(load_docs().await));
    let announcements: Arc<RwLock<Vec<AnnouncementPayload>>> =
        Arc::new(RwLock::new(load_announcements().await));

    // Shared app state for SSR
    let shared_app_state = ssr::setup_app_state_ssr().await;

    // Create router
    let app = create_router(docs, announcements, auth_layer, shared_app_state, pool);

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
            let config = RustlsConfig::from_pem_file(
                GLOBAL_CONFIG.https_cert_path.clone(),
                GLOBAL_CONFIG.https_key_path.clone(),
            )
            .await
            .unwrap();

            let addr = format!(
                "{}:{}",
                GLOBAL_CONFIG.ip_addr.clone(),
                GLOBAL_CONFIG.https_port
            )
            .parse::<SocketAddr>()
            .unwrap();
            info!("listening on https://{}", addr);

            if GLOBAL_CONFIG.redirect_to_https {
                tokio::spawn(redirect_http_to_https());
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

use std::{collections::HashMap, sync::Arc};

use axum_login::{
    tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder,
};
use payloads::AnnouncementPayload;
use routes::create_router;
use sessions::Backend;
use tokio::{net::TcpListener, sync::RwLock};
use utils::{load_announcements, load_docs};

mod handlers;
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

    // Initalize DB pool
    let _database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

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
    let app = create_router(docs, announcements, auth_layer, shared_app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

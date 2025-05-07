use tracing::error;

mod config;
mod db;
mod handlers;
mod init;
mod payloads;
mod routes;
mod sessions;

#[tokio::main]
async fn main() {
    // Load dotenv file
    dotenvy::dotenv().ok();

    // Logging/Tracing setup
    tracing_subscriber::fmt::init();

    init::init().await.unwrap_or_else(|e| {
        error!("Backend terminated with error: {}", e);
        std::process::exit(1);
    });
}

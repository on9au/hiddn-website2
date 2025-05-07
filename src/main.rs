use tracing::error;

#[tokio::main]
async fn main() {
    // Load dotenv file
    dotenvy::dotenv().ok();

    // Logging/Tracing setup
    tracing_subscriber::fmt::init();

    hiddn_website::init::init().await.unwrap_or_else(|e| {
        error!("Backend terminated with error: {}", e);
        std::process::exit(1);
    });
}

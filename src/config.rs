use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

#[derive(Debug, Deserialize)]
pub struct Config {
    // HTTP/HTTPS Config
    pub http_or_https: HttpOrHttps,
    // HTTP Config
    pub redirect_to_https: bool,
    pub http_socket_addr: String,
    // HTTPS Config
    pub https_socket_addr: String,
    pub https_cert_path: String,
    pub https_key_path: String,

    // Database Config
    pub database_url: String,

    // Stripe Config
    pub stripe_api_key: String,

    // Announcements and Documentation Config
    pub announcements_dir: String,
    pub documentation_dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HttpOrHttps {
    Http,
    Https,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        envy::from_env().unwrap()
    }
}

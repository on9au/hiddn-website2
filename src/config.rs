use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

#[derive(Debug, Deserialize)]
pub struct Config {
    // HTTP/HTTPS Config
    pub http_or_https: HttpOrHttps,
    pub ip_addr: String,

    // HTTP Config
    pub http_port: u16,
    pub redirect_to_https: bool,

    // HTTPS Config
    pub https_port: u16,
    pub https_cert_path: String,
    pub https_key_path: String,

    // Express Config
    pub express_port: u16,

    // Database Config
    pub database_url: String,

    // Marzban Panel Config
    pub marzban_panel_url: String,
    pub marzban_panel_username: String,
    pub marzban_panel_password: String,

    // Stripe Config
    pub stripe_secret_key: String,

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

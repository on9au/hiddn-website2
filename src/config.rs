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

    // Email Client Config
    pub from_email: String,
    pub default_subject: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,

    // Template Config
    pub templates_dir: String,

    // Database Config
    pub database_url: String,

    // Marzban Panel Config
    pub marzban_panel_url: String,
    pub marzban_panel_username: String,
    pub marzban_panel_password: String,

    // Stripe Config
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,

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

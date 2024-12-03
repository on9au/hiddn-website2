use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

#[derive(Debug, Deserialize)]
pub struct Config {
    pub socket_addr: String,
    pub database_url: String,
    pub stripe_api_key: String,
    pub announcements_dir: String,
    pub documentation_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        envy::from_env().unwrap()
    }
}

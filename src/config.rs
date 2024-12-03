use once_cell::sync::Lazy;
use serde::Deserialize;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(Config::from_env);

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub socket_addr: String,
    pub stripe_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        envy::from_env().unwrap()
    }
}

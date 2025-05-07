use marzban_api::client::MarzbanAPIClient;
use sqlx::MySqlPool;
use tera::Tera;

pub struct AppState {
    pub db_pool: MySqlPool,
    pub marzban_client: MarzbanAPIClient,
    pub stripe_client: stripe::Client,
    pub tera: Tera,
}

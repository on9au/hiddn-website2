use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct AdminUser {
    pub id: u32,
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct AdminUserModify {
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct AdminUserCreate {
    pub email: String,
    pub marzban_username: Option<String>,
    pub password: String,
    pub admin: bool,
}

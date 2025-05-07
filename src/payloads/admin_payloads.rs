use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct AdminUser {
    pub id: u32,
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct AdminUserModify {
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
}

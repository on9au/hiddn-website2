use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct UserProfilePayload {
    pub email: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub email_expiration_reminder: bool,
    pub email_data_reminder: bool,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct UserProfileSettingsChangePayload {
    pub email_expiration_reminder: Option<bool>,
    pub email_data_reminder: Option<bool>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct UserProfile {
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email_expiration_reminder: bool,
    pub email_data_reminder: bool,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct UserProfileSettingsChange {
    pub email_expiration_reminder: Option<bool>,
    pub email_data_reminder: Option<bool>,
}

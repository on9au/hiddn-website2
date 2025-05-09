use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct Announcement {
    pub id: u32,
    pub title: String,
    pub date: DateTime<Utc>,
    pub content: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct AdminCreateAnnouncement {
    pub title: String,
    pub content: String,
}

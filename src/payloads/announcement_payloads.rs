use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct AnnouncementPayload {
    pub id: u64,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct AdminCreateAnnouncement {
    pub title: String,
    pub content: String,
}

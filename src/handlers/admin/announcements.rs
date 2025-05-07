use crate::db::AnnouncementRepository;
use axum::{Json, response::IntoResponse};

pub async fn admin_post_announcement(/* params */) -> impl IntoResponse {
    Json("stub: admin_post_announcement")
}

pub async fn admin_delete_announcement(/* params */) -> impl IntoResponse {
    Json("stub: admin_delete_announcement")
}

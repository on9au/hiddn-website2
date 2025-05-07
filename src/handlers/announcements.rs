use crate::db::PlanRepository;
use axum::{Json, response::IntoResponse};

pub async fn get_announcements(/* params */) -> impl IntoResponse {
    Json("stub: get_announcements")
}

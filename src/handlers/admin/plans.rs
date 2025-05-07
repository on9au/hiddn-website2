use crate::db::PlanRepository;
use axum::{Json, response::IntoResponse};

pub async fn admin_create_plan(/* params */) -> impl IntoResponse {
    Json("stub: admin_create_plan")
}

pub async fn admin_delete_plan(/* params */) -> impl IntoResponse {
    Json("stub: admin_delete_plan")
}

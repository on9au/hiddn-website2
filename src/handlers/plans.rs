use crate::db::PlanRepository;
use axum::{Json, response::IntoResponse};

pub async fn get_plans(/* params */) -> impl IntoResponse {
    // PlanRepository::get_plans(...)
    Json("stub: get_plans")
}

pub async fn get_plan_by_id(/* params */) -> impl IntoResponse {
    Json("stub: get_plan_by_id")
}

use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn get_plans(/* params */) -> AppResult<impl IntoResponse> {
    // PlanRepository::get_plans(...)
    Ok(Json("stub: get_plans"))
}

pub async fn get_plan_by_id(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: get_plan_by_id"))
}

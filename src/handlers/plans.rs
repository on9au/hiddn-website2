use crate::{errors::AppResult, state::AppState};
use axum::http::StatusCode;
use axum::{Extension, Json, response::IntoResponse};

pub async fn get_plans(Extension(app_state): Extension<AppState>) -> AppResult<impl IntoResponse> {
    let plans = app_state.plan_repository().get_plans().await?;
    Ok(Json(plans))
}

pub async fn get_plan_by_id(
    Extension(app_state): Extension<AppState>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> AppResult<impl IntoResponse> {
    match app_state
        .plan_repository()
        .get_plan_by_id(id as i64)
        .await?
    {
        Some(plan) => Ok(Json(plan).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Plan not found").into_response()),
    }
}

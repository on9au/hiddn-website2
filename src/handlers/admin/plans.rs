use crate::{errors::AppResult, payloads::NewPlan, state::AppState};
use anyhow::Context;
use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};

/// POST `/api/admin/plans`
pub async fn admin_create_plan(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<NewPlan>,
) -> AppResult<impl IntoResponse> {
    let id = app_state.plan_repository().create_plan(&payload).await?;

    // Return the plan as JSON
    let plan = app_state
        .plan_repository()
        .get_plan_by_id(id)
        .await?
        .context("Failed to get plan")?;
    Ok(Json(plan).into_response())
}

/// DELETE `/api/admin/plans/:id`
pub async fn admin_delete_plan(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    if app_state.plan_repository().delete_plan(id).await? {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

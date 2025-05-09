use crate::{errors::AppResult, handlers::app, payloads::AdminCreateAnnouncement, state::AppState};
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use reqwest::StatusCode;

/// POST `/api/admin/announcements/`
pub async fn admin_post_announcement(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<AdminCreateAnnouncement>,
) -> AppResult<impl IntoResponse> {
    let title = payload.title;
    let content = payload.content;
    app_state
        .announcement_repository()
        .create_announcement(title, content)
        .await?;

    let latest_announcement = app_state
        .announcement_repository()
        .get_latest_announcement()
        .await?
        .unwrap(); // we just created it, so it should exist

    Ok(Json(latest_announcement))
}

pub async fn admin_delete_announcement(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let deleted = app_state
        .announcement_repository()
        .delete_announcement(id as i64)
        .await?;

    if !deleted {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

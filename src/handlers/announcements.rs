use crate::{errors::AppResult, state::AppState};
use axum::{Extension, Json, response::IntoResponse};

pub async fn get_announcements(
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let announcements = app_state
        .announcement_repository()
        .get_announcements()
        .await?;

    Ok(Json(announcements))
}

use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::errors::AppResult;

pub mod announcements;
pub mod plans;
pub mod transactions;
pub mod users;

/// GET `/api/admin/me`
pub async fn me() -> AppResult<impl IntoResponse> {
    // admin route is already protected by middleware
    // so we don't need to check for admin here
    Ok(StatusCode::OK.into_response())
}

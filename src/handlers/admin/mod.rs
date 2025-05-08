use axum::{Json, response::IntoResponse};

use crate::errors::AppResult;

pub mod announcements;
pub mod plans;
pub mod transactions;
pub mod users;

/// GET `/api/admin/me`
pub async fn me() -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_me"))
}

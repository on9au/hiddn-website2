use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn get_announcements(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: get_announcements"))
}

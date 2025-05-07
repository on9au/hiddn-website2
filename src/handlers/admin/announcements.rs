use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn admin_post_announcement(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_post_announcement"))
}

pub async fn admin_delete_announcement(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_delete_announcement"))
}

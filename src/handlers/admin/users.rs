use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn admin_get_users(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_get_users"))
}

pub async fn admin_create_user(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_create_user"))
}

pub async fn admin_get_user(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_get_user"))
}

pub async fn admin_update_user(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_update_user"))
}

pub async fn admin_delete_user(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_delete_user"))
}

use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn admin_create_plan(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_create_plan"))
}

pub async fn admin_delete_plan(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_delete_plan"))
}

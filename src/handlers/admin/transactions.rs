use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn admin_get_transactions(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_get_transactions"))
}

pub async fn admin_get_transaction(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: admin_get_transaction"))
}

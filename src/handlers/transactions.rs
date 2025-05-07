use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn get_transactions(/* params */) -> AppResult<impl IntoResponse> {
    // TransactionRepository::get_transactions(...)
    Ok(Json("stub: get_transactions"))
}

pub async fn create_transaction(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: create_transaction"))
}

pub async fn get_transaction_by_id(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: get_transaction_by_id"))
}

pub async fn get_transaction_secret(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: get_transaction_secret"))
}

pub async fn complete_transaction(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: complete_transaction"))
}

pub async fn cancel_transaction(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: cancel_transaction"))
}

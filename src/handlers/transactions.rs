use crate::db::TransactionRepository;
use axum::{Json, response::IntoResponse};

pub async fn get_transactions(/* params */) -> impl IntoResponse {
    // TransactionRepository::get_transactions(...)
    Json("stub: get_transactions")
}

pub async fn create_transaction(/* params */) -> impl IntoResponse {
    Json("stub: create_transaction")
}

pub async fn get_transaction_by_id(/* params */) -> impl IntoResponse {
    Json("stub: get_transaction_by_id")
}

pub async fn get_transaction_secret(/* params */) -> impl IntoResponse {
    Json("stub: get_transaction_secret")
}

pub async fn complete_transaction(/* params */) -> impl IntoResponse {
    Json("stub: complete_transaction")
}

pub async fn cancel_transaction(/* params */) -> impl IntoResponse {
    Json("stub: cancel_transaction")
}

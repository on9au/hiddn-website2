use crate::db::TransactionRepository;
use axum::{Json, response::IntoResponse};

pub async fn admin_get_transactions(/* params */) -> impl IntoResponse {
    Json("stub: admin_get_transactions")
}

pub async fn admin_get_transaction(/* params */) -> impl IntoResponse {
    Json("stub: admin_get_transaction")
}

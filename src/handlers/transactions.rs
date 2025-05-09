use crate::{errors::AppResult, state::AppState};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json, response::IntoResponse};

pub async fn get_transactions(
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let txs = app_state
        .transaction_repository()
        .get_transactions()
        .await?;
    Ok(Json(txs))
}

pub async fn get_transaction_by_id(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    match app_state
        .transaction_repository()
        .get_transaction_by_id(id as i64)
        .await?
    {
        Some(tx) => Ok(Json(tx).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Transaction not found").into_response()),
    }
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

pub async fn create_transaction(/* params */) -> AppResult<impl IntoResponse> {
    Ok(Json("stub: create_transaction"))
}

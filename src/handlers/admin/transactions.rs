use crate::{errors::AppResult, state::AppState};
use axum::{Extension, Json, extract::Path, response::IntoResponse};

/// GET `/api/admin/transactions`
pub async fn admin_get_transactions(
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    // Return ALL transactions
    let all_transactions = app_state
        .transaction_repository()
        .get_transactions()
        .await?;

    // Return the transactions as JSON
    Ok(Json(all_transactions).into_response())
}

/// GET `/api/admin/transactions/:id`
pub async fn admin_get_transaction(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let transaction = app_state
        .transaction_repository()
        .get_transaction_by_id(id as i64)
        .await?;

    // Return the transaction as JSON
    Ok(Json(transaction).into_response())
}

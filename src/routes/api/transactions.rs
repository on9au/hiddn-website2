//! # `/api/transactions/` API Routes
//!
//! This module contains the API routes for the transactions.

use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::transactions::*;

/// # `/api/transactions/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_transactions))
        .route("/", post(create_transaction))
        .route("/{:id}", get(get_transaction_by_id))
        .route("/{:id}/secret", get(get_transaction_secret))
        .route("/{:id}/complete", get(complete_transaction))
        .route("/{:id}/cancel", get(cancel_transaction))
}

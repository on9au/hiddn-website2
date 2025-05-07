//! # `/api/admin/transactions/` API Routes
//!
//! This module contains the API routes for the transactions.

use axum::{Router, routing::get};

use crate::handlers::admin::transactions;

/// # `/api/admin/transactions/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(transactions::admin_get_transactions))
        .route("/{:id}", get(transactions::admin_get_transaction))
}

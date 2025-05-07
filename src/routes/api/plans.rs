//! # `/api/plans/` API Routes
//!
//! This module contains the API routes for the plans.

use axum::{Router, routing::get};

use crate::handlers::plans::*;

/// # `/api/plans/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_plans))
        .route("/:id", get(get_plan_by_id))
}

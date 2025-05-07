//! # `/api/admin/plans/` API Routes
//!
//! This module contains the API routes for the plans in the admin panel.

use axum::{
    Router,
    routing::{delete, post},
};

use crate::handlers::admin::plans;

/// # `/api/admin/plans/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", post(plans::admin_create_plan))
        .route("/:id", delete(plans::admin_delete_plan))
}

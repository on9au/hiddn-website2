//! # `/api/admin/announcements/` API Routes
//!
//! This module contains the API routes for the documentation for the admin panel.

use axum::{
    Router,
    routing::{delete, post},
};

use crate::handlers::admin::announcements;

/// # `/api/admin/announcements/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", post(announcements::admin_post_announcement))
        .route("/:id", delete(announcements::admin_delete_announcement))
}

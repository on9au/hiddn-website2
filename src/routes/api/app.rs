//! # `/api/app/` API Routes
//!
//! This module contains the API routes for the documentation.

use axum::{Router, routing::get};

use crate::handlers::app;

/// # `/api/app/` API Routes
pub fn routes() -> Router {
    Router::new().route("/config", get(app::get_clash_meta_config))
}

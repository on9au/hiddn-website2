//! # `/api/announcements/` API Routes
//!
//! This module contains the API routes for the documentation.

use axum::{Router, routing::get};

use crate::handlers::announcements;

/// # `/api/announcements/` API Routes
pub fn routes() -> Router {
    Router::new().route("/", get(announcements::get_announcements))
}

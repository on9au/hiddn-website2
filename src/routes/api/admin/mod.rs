//! # `/api/admin/` API Routes
//!
//! This module contains the API routes for the admin panel.
//!
//! This module requires admin authentication and will have middleware to check for admin authentication.
//!
//! ## Nested Routes
//!
//! - [`announcements`]: Announcement routes
//! - [`plans`]: Plan routes
//! - [`transactions`]: Transaction routes
//! - [`users`]: Me routes

use axum::{Router, routing::get};

use crate::handlers::admin;

pub mod announcements;
pub mod plans;
pub mod transactions;
pub mod users;

/// # `/api/admin/` API Routes
pub fn routes() -> Router {
    Router::new()
        .nest("/announcements", announcements::routes())
        .nest("/plans", plans::routes())
        .nest("/transactions", transactions::routes())
        .nest("/users", users::routes())
        .route("/me", get(admin::me))
}

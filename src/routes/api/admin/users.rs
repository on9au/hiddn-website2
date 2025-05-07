//! # `/api/admin/users/` API Routes
//!
//! This module contains the API routes for the users which are used in the admin panel.

use axum::{
    Router,
    routing::{delete, get, put},
};

use crate::handlers::admin::users;

/// # `/api/admin/users/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(users::admin_get_users))
        .route("/", put(users::admin_create_user))
        .route("/{:id}", get(users::admin_get_user))
        .route("/{:id}", put(users::admin_update_user))
        .route("/{:id}", delete(users::admin_delete_user))
}

//! # `/api/me` API Routes
//!
//! This module contains the API routes for the user.

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::handlers::me;

/// # `/api/me` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(me::get_me))
        .route("/reset-subscription-url", post(me::reset_subscription_url))
        .route("/update-settings", post(me::update_settings))
        .route("/delete-account", delete(me::delete_account))
        .route("/change-password", post(me::change_password))
}

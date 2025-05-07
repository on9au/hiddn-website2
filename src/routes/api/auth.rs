//! # `/api/auth/` API Routes
//!
//! This module contains the API routes for authentication.
//!
//! This is a public module, meaning that it does not require authentication.

use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::auth;

/// # `/api/auth/` API Routes
pub fn routes() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/is-logged-in", get(auth::is_logged_in))
        .route("/register", post(auth::register_user))
        .route("/forgot-password", post(auth::forgot_password))
        .route("/email/request-code", post(auth::request_code))
        .route("/email/verify", post(auth::verify_email))
}

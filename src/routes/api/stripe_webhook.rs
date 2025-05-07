//! # `/api/stripe/` API Routes
//!
//! This module contains the API routes for handling Stripe webhooks.
//!
//! This is a special route that is used to handle Stripe webhooks.

use axum::{Router, routing::post};

use crate::handlers::stripe_webhook;

/// # `/api/auth/` API Routes
pub fn routes() -> Router {
    Router::new().route("/", post(stripe_webhook::stripe_webhook))
}

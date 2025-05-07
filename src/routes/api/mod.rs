//! # `/api/` API Routes
//!
//! This module contains the API routes for the backend.
//!
//! ## Nested Routes
//!
//! Public:
//!
//! - [`auth`]: Authentication routes
//!
//! Webhook API:
//!
//! - [`stripe`]: Stripe (webhook) routes
//!
//! Protected:
//!
//! - [`admin`]: Admin routes
//! - [`announcements`]: Announcement routes
//! - [`app`]: App routes
//! - [`me`]: Me routes
//! - [`plans`]: Plan routes
//! - [`transactions`]: Transaction routes

pub mod admin;
pub mod announcements;
pub mod app;
pub mod auth;
pub mod me;
pub mod plans;
pub mod stripe_webhook;
pub mod transactions;

use std::sync::Arc;

use axum::{Extension, Router};
use axum_login::{AuthManagerLayer, login_required, tower_sessions::MemoryStore};
use tower_http::trace::TraceLayer;

use crate::{sessions::Backend, state::AppState};

pub fn routes(auth_layer: AuthManagerLayer<Backend, MemoryStore>, app_state: AppState) -> Router {
    // Public routes which will lack the auth_layer.
    let public_routes = Router::new()
        .nest("/auth", auth::routes())
        .nest("/stripe", stripe_webhook::routes());

    // Protected routes which will have the auth_layer applied.
    let protected_routes = Router::new()
        .nest("/admin", admin::routes())
        .nest("/announcements", announcements::routes())
        .nest("/app", app::routes())
        .nest("/me", me::routes())
        .nest("/plans", plans::routes())
        .nest("/transactions", transactions::routes())
        .route_layer(login_required!(Backend));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(Arc::new(app_state)))
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
}

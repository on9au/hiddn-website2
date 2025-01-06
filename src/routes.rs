use std::{collections::HashMap, sync::Arc};

use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use axum_login::{login_required, tower_sessions::MemoryStore, AuthManagerLayer};
use marzban_api::client::MarzbanAPIClient;
use tokio::sync::RwLock;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    handlers::*,
    handlers_admin::{
        admin_me, admin_my_id, admin_user, admin_users, delete_announcement, delete_plan,
        delete_user, post_announcement, post_plan, update_user,
    },
    payloads::AnnouncementPayload,
    sessions::Backend,
};

pub fn create_router(
    docs: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    announcements: Arc<RwLock<Vec<AnnouncementPayload>>>,
    auth_layer: AuthManagerLayer<Backend, MemoryStore>,
    pool: sqlx::MySqlPool,
    marzban_client: MarzbanAPIClient,
    stripe_client: stripe::Client,
) -> Router {
    Router::new()
        // API Routes
        .nest(
            "/api",
            Router::new() // Protected routes
                .route("/documentation", get(get_documentation))
                .route("/documentation/options", get(list_documentation_options))
                .route(
                    "/documentation/categories",
                    get(get_documentation_categories),
                )
                .layer(Extension(docs))
                .route("/announcements", get(get_announcements))
                // Admin routes for announcements
                .route("/admin/announcements", post(post_announcement))
                .route("/admin/announcements/:id", delete(delete_announcement))
                .layer(Extension(announcements))
                .route("/transactions", get(transactions))
                .route("/transaction/:id", get(transactions_id))
                .route("/transaction/:id/secret", get(transaction_secret))
                .route("/transaction/:id/complete", post(transaction_complete))
                .route("/transaction/:id/cancel", post(transaction_cancel))
                .route("/server_status", get(server_status))
                .route("/plan_details", get(plan_details))
                .route("/plans", get(plans))
                .route("/plans/:id", get(plans_id))
                .route("/orders", post(create_transaction))
                .route("/reset_subscription_url", post(reset_subscription_url))
                .route("/update_settings", post(update_settings))
                .route("/delete_account", delete(delete_account))
                .route("/change_password", post(change_password))
                .route("/me", get(user_me))
                // Admin routes
                .route("/admin/me", get(admin_me))
                .route("/admin/my_id", get(admin_my_id))
                .route("/admin/users", get(admin_users))
                .route("/admin/users/:id", get(admin_user))
                .route("/admin/users/:id", put(update_user))
                .route("/admin/users/:id", delete(delete_user))
                .route("/admin/plans", post(post_plan))
                .route("/admin/plans/:id", delete(delete_plan))
                .route_layer(login_required!(Backend))
                // Routes involving authentication
                .route("/", get(root))
                .route("/login_user", post(login_user))
                .route("/logout_user", post(logout_user))
                .route("/is_logged_in", get(is_logged_in))
                .layer(auth_layer)
                // Unprotected routes
                .route("/register_user", post(register_user))
                .route("/forgot_password", post(forgot_password))
                .route("/generate_204", get(generate_204))
                .route("/request_code", post(request_code))
                .route("/verify_email", post(verify_email))
                .route("/stripe", post(stripe_webhook))
                .layer(Extension(pool))
                .layer(Extension(marzban_client))
                .layer(Extension(stripe_client))
                .layer(TraceLayer::new_for_http()),
        )
        // SSR Frontend
        .nest_service(
            "/",
            ServeDir::new("static"),
        )
}

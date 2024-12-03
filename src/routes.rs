use std::{collections::HashMap, sync::Arc};

use axum::{
    handler::HandlerWithoutStateExt,
    routing::{delete, get, post},
    Extension, Router,
};
use axum_login::{login_required, tower_sessions::MemoryStore, AuthManagerLayer};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

use crate::{
    handlers::*,
    handlers_admin::{admin_me, admin_users, delete_announcement, post_announcement},
    payloads::AnnouncementPayload,
    sessions::Backend,
    ssr::{handle_ssr, AppState},
};

pub fn create_router(
    docs: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    announcements: Arc<RwLock<Vec<AnnouncementPayload>>>,
    auth_layer: AuthManagerLayer<Backend, MemoryStore>,
    shared_app_state: Arc<AppState>,
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
                .route("/admin/announcements", post(post_announcement))
                .route("/admin/announcements/:id", delete(delete_announcement))
                .layer(Extension(announcements))
                .route("/transactions", get(transactions))
                .route("/transaction/:id", get(transactions_id))
                .route("/transaction/:id/complete", post(transaction_complete))
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
                .route("/admin/users", get(admin_users))
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
                .route("/verify_email", post(verify_email)),
        )
        // SSR Frontend
        .nest_service(
            "/",
            ServeDir::new("./client/dist/client")
                .append_index_html_on_directories(false)
                .fallback(get(handle_ssr).into_service()),
        )
        .layer(Extension(shared_app_state))
}

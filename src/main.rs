use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use axum_login::{
    login_required,
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use chrono::{DateTime, Utc};
use payloads::{
    AnnouncementPayload, ForgotPasswordPayload, LoginPayload, LoginResponsePayload,
    PlanDetailsPayload, PlanPayload, PlanStatusEnum, RegisterPayload, ServerStatusPayload,
    UserTransactionPayload, UserTransactionStatusEnum, VerifyEmailPayload,
};
use serde_json::json;
use sessions::{AuthSession, Backend};
use tokio::{fs, net::TcpListener, sync::RwLock};

mod payloads;
mod sessions;

type SharedDocs = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

#[tokio::main]
async fn main() {
    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::default();
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let docs = Arc::new(RwLock::new(load_docs().await));

    let announcements = Arc::new(RwLock::new(load_announcements().await));

    let app = Router::new()
        // Protected routes
        .route("/documentation", get(get_documentation))
        .route("/documentation/options", get(list_documentation_options))
        .route(
            "/documentation/categories",
            get(get_documentation_categories),
        )
        .layer(Extension(docs))
        .route("/announcements", get(get_announcements))
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
        .route("/verify_email", post(verify_email));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn load_docs() -> HashMap<String, HashMap<String, String>> {
    let mut docs = HashMap::new();

    let mut paths = fs::read_dir("./docs").await.unwrap();

    let mut dir_entries = Vec::new();
    while let Some(entry) = paths.next_entry().await.unwrap() {
        dir_entries.push(entry);
    }

    for entry in dir_entries {
        println!("{:?}", entry.file_name());
        let os_name = entry.file_name().to_string_lossy().to_lowercase();
        let mut os_docs = HashMap::new();

        let os_path = entry.path();
        let mut files = fs::read_dir(os_path).await.unwrap();

        let mut file_entries = Vec::new();
        while let Some(file) = files.next_entry().await.unwrap() {
            file_entries.push(file);
        }

        for file in file_entries {
            println!("{:?}", file.file_name());
            let file_name = file.file_name().to_string_lossy().to_lowercase();
            if file_name.ends_with(".md") {
                let category = file_name.trim_end_matches(".md").to_string();
                let content = fs::read_to_string(file.path()).await.unwrap_or_default();
                os_docs.insert(category, content);
            }
        }

        docs.insert(os_name, os_docs);
    }

    println!("{:?}", docs);

    docs
}

async fn load_announcements() -> Vec<AnnouncementPayload> {
    let mut announcements = Vec::new();

    // Read announcements dir
    let mut paths = fs::read_dir("./announcements").await.unwrap();

    let mut dir_entries = Vec::new();

    while let Some(entry) = paths.next_entry().await.unwrap() {
        dir_entries.push(entry);
    }

    let mut entries_with_metadata: Vec<_> =
        futures::future::join_all(dir_entries.into_iter().map(|entry| async {
            let metadata = entry.metadata().await.unwrap();
            let modified = metadata.modified().unwrap();
            (entry, modified)
        }))
        .await;

    entries_with_metadata.sort_by_key(|&(_, modified)| std::cmp::Reverse(modified));

    let dir_entries: Vec<_> = entries_with_metadata
        .into_iter()
        .map(|(entry, _)| entry)
        .collect();

    for entry in dir_entries {
        println!("{:?}", entry.file_name());
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        if file_name.ends_with(".md") {
            let content = fs::read_to_string(entry.path()).await.unwrap_or_default();
            let announcement = AnnouncementPayload {
                id: (announcements.len() as u32).into(),
                title: file_name.trim_end_matches(".md").to_string(),
                date: DateTime::<Utc>::from(entry.metadata().await.unwrap().modified().unwrap())
                    .to_rfc3339()
                    .to_string(),
                content,
            };
            announcements.push(announcement);
        }
    }

    println!("{:?}", announcements);

    announcements
}

/// Handler for the GET `/` route.
async fn root() -> &'static str {
    "Hello, World!"
}

/// Handler for the GET '/generate_204' route.
/// This handler will return an empty response with a status code of NO_CONTENT.
/// This is to check if the user is connected to the internet, and if the API endpoint is live.
async fn generate_204() -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

/// Handler for the GET '/is_logged_in' route.
/// This handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// The server will use axum_login to keep the user authenticated.
async fn is_logged_in(auth_session: AuthSession) -> impl IntoResponse {
    println!("{:?}", auth_session.user);
    match auth_session.user {
        Some(_) => StatusCode::OK.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

/// Handler for the POST '/login_user' route.
/// This handler will receive a JSON(LoginPayload) payload from the client.
/// The handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// OK will be accompanied by Json(LoginResponsePayload) saying the user is authenticated.
/// If email is invalid, it will return BAD_REQUEST.
/// The server will use axum_login to keep the user authenticated.
async fn login_user(
    mut auth_session: AuthSession,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // TODO: Implement actual authentication logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    let user = match auth_session.authenticate(payload).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(e) => {
            println!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match auth_session.login(&user).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    (
        StatusCode::OK,
        Json(LoginResponsePayload { logged_in: true }),
    )
        .into_response()
}

/// Handler for the POST '/logout_user' route.
/// This handler will log the user out, clearing the session.
/// axum_login will handle the session management, logging the user out.
async fn logout_user(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            println!("Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler for the POST '/verify_email' route.
/// This handler will receive a JSON(VerifyEmailPayload) payload from the client.
/// It will send code to email to verify the email.
/// Should have a rate limit to prevent spamming.
/// This acts as a way to verify the email's ownership and existence.
async fn verify_email(Json(payload): Json<VerifyEmailPayload>) -> impl IntoResponse {
    // TODO: Implement actual email verification logic
    // We would create a temporary verificaton code linked to the email in the db which expires after a certain time.
    // We would send the verification code to the email.

    // Debug print the payload
    println!("{:?}", payload);

    StatusCode::OK.into_response()
}

/// Handler for the POST '/register_user' route.
/// This handler will receive a JSON(RegisterPayload) payload from the client.
/// The handler will return OK if the user is registered.
/// If email is already taken, it will return BAD_REQUEST.
/// If email is invalid, it will return BAD_REQUEST.
/// If password is invalid, it will return CONFLICT.
/// If password is too weak, it will return CONFLICT.
/// If verification code is invalid, it will return FORBIDDEN.
/// The server will use axum_login to keep the user authenticated.
async fn register_user(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    // TODO: Implement actual registration logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    // Check if the email is already taken
    println!("Checking if email is already taken...");

    // Check if code is valid
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Check if password is valid
    // Check if password is too weak
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    // We can use regex to check this.
    // We can also use a library like zxcvbn to check password strength.
    // For now, we will just check if the password is at least 8 characters long.
    if payload.password.len() < 8 {
        return StatusCode::CONFLICT.into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return StatusCode::CONFLICT.into_response();
    }

    // Register the user
    // We would hash the password before storing it in the db.
    // Verification code can be safely discarded after registration.
    // New db entry for the user. After, use axum_login to authenticate the user.
    // db new user
    // let registered_user = that db entry into User struct

    // Return OK, user is registered, client must now login.
    StatusCode::OK.into_response()
}

/// Handler for the POST '/forgot_password' route.
/// This handler will receive a JSON(ForgotPassword) payload from the client.
/// The handler will return OK if the user password is reset.
/// If user does not exist, it will return NOT_FOUND.
/// If password is invalid, it will return CONFLICT.
/// If password is too weak, it will return CONFLICT.
/// If verification code is invalid, it will return FORBIDDEN.
/// The server will use axum_login to keep the user authenticated.
async fn forgot_password(Json(payload): Json<ForgotPasswordPayload>) -> impl IntoResponse {
    // TODO: Implement actual registration logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    // Check if the user exists
    if payload.email != "test@test.com" {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Check if code is valid
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Check if password is valid
    // Check if password is too weak
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    // We can use regex to check this.
    // We can also use a library like zxcvbn to check password strength.
    // For now, we will just check if the password is at least 8 characters long.
    if payload.password.len() < 8 {
        return StatusCode::CONFLICT.into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return StatusCode::CONFLICT.into_response();
    }

    // Register the user
    // We would hash the password before storing it in the db.
    // Verification code can be safely discarded after registration.
    // New db entry for the user. After, use axum_login to authenticate the user.
    // db new user
    // let registered_user = that db entry into User struct

    // Return OK, user is registered, client must now login.
    StatusCode::OK.into_response()
}

/// Handler for the GET '/documentation' route.
/// This handler will return the documentation for the given OS and category.
/// The documentation is stored in a shared state.
/// The shared state is a HashMap<String, HashMap<String, String>>.
/// The outer HashMap is keyed by OS.
/// The inner HashMap is keyed by category.
/// The value is the documentation content.
/// The handler will return NOT_FOUND if the documentation is not found.
/// The handler will return the documentation content if found.
/// This handler requires authentication (managed by axum_login).
async fn get_documentation(
    Query(params): Query<HashMap<String, String>>,
    Extension(docs): Extension<SharedDocs>,
) -> impl IntoResponse {
    let os = params
        .get("os")
        .unwrap_or(&"common".to_string())
        .to_lowercase();
    let category = params
        .get("category")
        .unwrap_or(&"install".to_string())
        .to_lowercase();

    let docs = docs.read().await;

    if let Some(os_docs) = docs.get(&os) {
        if let Some(content) = os_docs.get(&category) {
            return (StatusCode::OK, content.clone()).into_response();
        }
    }

    (StatusCode::NOT_FOUND, "Documentation not found").into_response()
}

/// Handler for the GET '/documentation/options' route.
/// This handler will return a list of OS and categories for the documentation.
/// The documentation is stored in a shared state.
/// The shared state is a HashMap<String, HashMap<String, String>>.
/// The outer HashMap is keyed by OS.
/// The inner HashMap is keyed by category.
/// This handler requires authentication (managed by axum_login).
async fn list_documentation_options(Extension(docs): Extension<SharedDocs>) -> impl IntoResponse {
    let docs = docs.read().await;

    let mut os_list: Vec<String> = docs.keys().cloned().collect();

    os_list.sort();

    let response = json!({
        "osList": os_list
    });

    Json(response)
}

/// Handler for the GET '/documentation/categories' route.
async fn get_documentation_categories(
    Query(params): Query<HashMap<String, String>>,
    Extension(docs): Extension<SharedDocs>,
) -> impl IntoResponse {
    let os = params
        .get("os")
        .unwrap_or(&"common".to_string())
        .to_lowercase();

    let docs = docs.read().await;

    if let Some(os_docs) = docs.get(&os) {
        let categories: Vec<String> = os_docs.keys().cloned().collect();
        let response = json!({ "categories": categories });
        return Json(response).into_response();
    }

    (StatusCode::NOT_FOUND, "OS not found").into_response()
}

/// Handler for the GET '/transactions' route.
/// This handler will return Json(Vec<UserTransactionPayload>)
/// This handler will return the transactions of the user.
/// This handler requires authentication (managed by axum_login).
async fn transactions() -> impl IntoResponse {
    let transactions: Vec<payloads::UserTransactionPayload> = vec![
        UserTransactionPayload {
            transaction_id: 1_u32.into(),
            amount: 100.0,
            transaction_date: "2021-01-01T00:00:00Z".to_string(),
            payment_method: Some("Credit Card".to_string()),
            status: UserTransactionStatusEnum::Completed,
            stripe_payment_intent_id: Some("pi_123456".to_string()),
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
            plan_id: Some(2_u32.into()),
            description: Some("Payment for Premium Plan".to_string()),
        },
        UserTransactionPayload {
            transaction_id: 2_u32.into(),
            amount: 200.0,
            transaction_date: "2021-01-01T00:00:00Z".to_string(), // Placeholder
            payment_method: Some("Cash".to_string()),
            status: UserTransactionStatusEnum::Pending,
            stripe_payment_intent_id: Some("pi_123456".to_string()),
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
            plan_id: Some(2_u32.into()),
            description: Some("Payment for Premium Plan".to_string()),
        },
        UserTransactionPayload {
            transaction_id: 3_u32.into(),
            amount: 300.0,
            transaction_date: "2021-01-01T00:00:00Z".to_string(), // Placeholder
            payment_method: None,
            status: UserTransactionStatusEnum::Unpaid,
            stripe_payment_intent_id: Some("pi_123456".to_string()),
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
            plan_id: Some(2_u32.into()),
            description: None,
        },
        UserTransactionPayload {
            transaction_id: 4_u32.into(),
            amount: 400.0,
            transaction_date: "2021-01-01T00:00:00Z".to_string(), // Placeholder
            payment_method: Some("Paypal".to_string()),
            status: UserTransactionStatusEnum::Cancelled,
            stripe_payment_intent_id: None,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
            plan_id: None,
            description: Some("Payment for Premium Plan".to_string()),
        },
        UserTransactionPayload {
            transaction_id: 5_u32.into(),
            amount: 500.0,
            transaction_date: "2021-01-01T00:00:00Z".to_string(), // Placeholder
            payment_method: Some("Credit Card".to_string()),
            status: UserTransactionStatusEnum::Failed,
            stripe_payment_intent_id: Some("pi_123456".to_string()),
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
            plan_id: Some(2_u32.into()),
            description: Some("Payment for Premium Plan".to_string()),
        },
    ];

    Json(transactions).into_response()
}

/// Handler for the GET '/transaction/:id' route.
/// This handler will return Json(UserTransactionPayload)
/// This handler will return the transaction with the given id.
/// This handler requires authentication (managed by axum_login).
async fn transactions_id(Path(id): Path<u32>) -> impl IntoResponse {
    let transaction = UserTransactionPayload {
        transaction_id: id.into(),
        amount: 100.0,
        transaction_date: "2021-01-01T00:00:00Z".to_string(),
        payment_method: Some("Credit Card".to_string()),
        status: UserTransactionStatusEnum::Pending,
        stripe_payment_intent_id: Some("pi_123456".to_string()),
        created_at: "2021-01-01T00:00:00Z".to_string(),
        updated_at: "2021-01-01T00:00:00Z".to_string(),
        plan_id: Some(2_u32.into()),
        description: Some("Payment for Premium Plan".to_string()),
    };

    Json(transaction).into_response()
}

/// Handler for the POST '/transaction/:id/complete' route.
/// This handler will complete the transaction with the given id.
/// This handler requires authentication (managed by axum_login).
async fn transaction_complete(Path(id): Path<u32>) -> impl IntoResponse {
    // Would complete the transaction in the db.
    // Would also complete the stripe payment intent.
    StatusCode::OK.into_response()
}

/// Handler for the GET '/server_status' route.
/// This handler will return Json(Vec<ServerStatusPayload>)
/// This handler will return the status of the servers.
/// This handler requires authentication (managed by axum_login).
async fn server_status() -> impl IntoResponse {
    let server_status: Vec<ServerStatusPayload> = vec![
        ServerStatusPayload {
            server: "Melbourne".to_string(),
            status: "Online".to_string(),
        },
        ServerStatusPayload {
            server: "Sydney".to_string(),
            status: "Degraded".to_string(),
        },
        ServerStatusPayload {
            server: "Singapore".to_string(),
            status: "Unreachable".to_string(),
        },
    ];

    Json(server_status).into_response()
}

/// Handler for the GET '/plan_details' route.
/// This handler will return Json(Option<PlanDetailsPayload>)
/// This handler will return the details of the user's plan.
/// This handler requires authentication (managed by axum_login).
async fn plan_details() -> impl IntoResponse {
    let plan_details = PlanDetailsPayload {
        expiration: "2021-01-01T00:00:00Z".to_string(), // Placeholder
        status: PlanStatusEnum::Active,
        data_used: 15.9,
        data_limit: 40.0,
    };

    Json(plan_details).into_response()
}

/// Handler for the GET '/plans' route.
/// This handler will return Json(Vec<PlanDetailsPayload>)
/// This handler will return all the plans available.
/// This handler requires authentication (managed by axum_login).
async fn plans() -> impl IntoResponse {
    let plans = vec![
        PlanPayload {
            id: 0_u32.into(),
            name: "Basic".to_string(),
            price: 5.0,
            data_limit: Some(20.0),
            duration_days: 30_u32.into(),
            description: Some("Basic plan".to_string()),
        },
        PlanPayload {
            id: 1_u32.into(),
            name: "Premium".to_string(),
            price: 10.0,
            data_limit: Some(40.0),
            duration_days: 30_u32.into(),
            description: Some("Premium plan".to_string()),
        },
    ];

    Json(plans).into_response()
}

/// Handler for the GET '/plans/:id' route.
/// This handler will return Json(PlanPayload)
/// This handler will return the details of the plan with the given id.
/// This handler requires authentication (managed by axum_login).
async fn plans_id(Path(id): Path<u32>) -> impl IntoResponse {
    let plan_details = PlanPayload {
        id: id.into(),
        name: "Premium".to_string(),
        price: 10.0,
        data_limit: Some(40.0),
        duration_days: 30_u32.into(),
        description: Some("Premium plan".to_string()),
    };

    Json(plan_details).into_response()
}

/// Handler for the POST '/orders' route.
/// This handler will create an order for the user.
/// This handler will return Json(CreateOrderResponsePayload)
/// This handler requires authentication (managed by axum_login).
async fn create_transaction() -> impl IntoResponse {
    // Would create a new order in the db.
    // Would also create stripe payment intent.
    Json(payloads::CreateOrderResponsePayload {
        order_id: 1_u32.into(),
        payment_intent_client_secret: "pi_123456".to_string(),
    })
    .into_response()
}

/// Handler for the POST '/reset_subscription_url' route.
/// This handler will reset the subscription URL.
/// This handler requires authentication (managed by axum_login).
async fn reset_subscription_url() -> impl IntoResponse {
    // Typically, would call marzban api to reset the subscription URL.
    StatusCode::OK.into_response()
}

/// Handler for the POST '/update_settings' route.
/// This handler will update the user's settings.
/// This handler requires authentication (managed by axum_login).
async fn update_settings() -> impl IntoResponse {
    // Typically, would update the user's settings in the db.
    StatusCode::OK.into_response()
}

/// Handler for the DELETE '/delete_account' route.
/// This handler will delete the user's account.
/// This handler requires authentication (managed by axum_login).
async fn delete_account() -> impl IntoResponse {
    // Typically, would delete the user's account in the db.
    StatusCode::OK.into_response()
}

/// Handler for the POST '/change_password' route.
/// This handler will change the user's password.
/// This handler requires authentication (managed by axum_login).
async fn change_password() -> impl IntoResponse {
    // Typically, would change the user's password in the db, as well as:
    // - Verify if the password is valid
    // - Invalidate all sessions
    // - Send an email to the user notifying them of the password change
    StatusCode::OK.into_response()
}

/// Handler for the GET '/announcements' route.
/// This handler will return Json(Vec<AnnouncementPayload>)
/// This handler will return the announcements.
/// This handler requires authentication (managed by axum_login).
async fn get_announcements(
    Extension(announcements): Extension<Arc<RwLock<Vec<AnnouncementPayload>>>>,
) -> impl IntoResponse {
    let announcements = announcements.read().await;

    Json(announcements.clone()).into_response()
}

/// Handler for the GET '/me' route.
/// This handler will return Json(UserProfilePayload)
/// This handler will return the user's profile.
/// This handler requires authentication (managed by axum_login).
async fn user_me(auth_session: AuthSession) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    let user_profile = payloads::UserProfilePayload {
        email: user.email,
        email_verified: true,
        created_at: "2021-01-01T00:00:00Z".to_string(), // Placeholder
        updated_at: "2021-01-01T00:00:00Z".to_string(),
        email_expiration_reminder: false,
        email_data_reminder: true, // Placeholder
    };

    Json(user_profile).into_response()
}

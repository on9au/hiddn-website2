use std::{collections::HashMap, sync::Arc};

use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequest, Request};
use futures::StreamExt;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use marzban_api::models::proxy::ProxyTypes;
use rand::Rng;
use tera::Tera;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;

use axum::response::{IntoResponse, Response};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};
use marzban_api::client::MarzbanAPIClient;
use marzban_api::models::user::{
    Inbounds, Proxies, Shadowsocks, Trojan, UserCreate, UserModify, UserStatus, UserStatusCreate,
    UserStatusModify, Vless, Vmess,
};
use num_traits::ToPrimitive;
use serde_json::json;
use sqlx::types::BigDecimal;
use sqlx::{MySqlPool, query};
use stripe::{CancelPaymentIntent, CreatePaymentIntent, EventObject, EventType, PaymentIntent};
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::config::GLOBAL_CONFIG;
use crate::payloads::{
    ChangePasswordPayload, CreateOrderPayload, PlanDetailsRust, UserProfileRust,
    UserProfileSettingsChangePayload, UserTransactionRust,
};
use crate::{
    SharedDocs,
    payloads::{
        AnnouncementPayload, CreateOrderResponsePayload, ForgotPasswordPayload, LoginPayload,
        LoginResponsePayload, PasswordFeedbackPayload, PlanPayload, PlanStatusEnum,
        RegisterPayload, RequestCodePayload, ServerStatusPayload, UserTransactionStatusEnum,
        VerifyEmailPayload,
    },
    sessions::AuthSession,
};

/// Handler for the GET `/` route.
pub async fn root() -> &'static str {
    "Hello, World!"
}

/// Handler for the GET '/generate_204' route.
/// This handler will return an empty response with a status code of NO_CONTENT.
/// This is to check if the user is connected to the internet, and if the API endpoint is live.
pub async fn generate_204() -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}

/// Handler for the GET '/is_logged_in' route.
/// This handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// The server will use axum_login to keep the user authenticated.
pub async fn is_logged_in(auth_session: AuthSession) -> impl IntoResponse {
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
pub async fn login_user(
    mut auth_session: AuthSession,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // Authenticate the user with the db
    let user = match auth_session.authenticate(payload).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(e) => {
            error!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Log the user in
    match auth_session.login(&user).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e);
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
pub async fn logout_user(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!("Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler for the POST '/verify_email' route.
/// This handler will receive a JSON(VerifyEmailPayload) payload from the client.
/// It will validate the email verification code.
/// This route should only be used if you need to just verify the email.
/// If you need to register or reset password, use the respective routes instead.
/// This acts as a way to verify the email's ownership and existence.
/// Returns OK if the email is verified, and FORBIDDEN if the code is invalid.
pub async fn verify_email(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<VerifyEmailPayload>,
) -> impl IntoResponse {
    // Check if code is valid
    let code_id = query!(
        r#"
        SELECT id
        FROM verification_codes
        WHERE email = ?
        AND code = ?
        AND is_used = false
        AND expires_at > NOW()
        "#,
        payload.email,
        payload.email_verification_code
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to check if code is valid");

    let code_id = match code_id {
        Some(code_id) => Some(code_id.id),
        None => return StatusCode::FORBIDDEN.into_response(),
    };

    // All checks passed, register the user and mark the verification code as used

    // Mark the code as used
    query!(
        r#"
        UPDATE verification_codes
        SET is_used = true
        WHERE id = ?
        "#,
        code_id
    )
    .execute(&pool)
    .await
    .expect("Failed to mark code as used");

    StatusCode::OK.into_response()
}

/// Handler for the POST `/request_code` route.
/// This handler will receive a JSON(RequestCodePayload) payload from the client.
/// It will send code to email to verify the email.
/// Should have a rate limit to prevent spamming.
/// This acts as a way to verify the email's ownership and existence.
pub async fn request_code(
    Extension(tera): Extension<Tera>,
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<RequestCodePayload>,
) -> impl IntoResponse {
    // Ensure that last code was sent at least 30 seconds ago
    // This is to prevent spamming the email
    let currently_valid_codes = query!(
        r#"
        SELECT created_at
        FROM verification_codes
        WHERE email = ?
        AND expires_at > NOW()
        AND is_used = false
        ORDER BY created_at DESC
        "#,
        payload.email
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch verification codes");

    // If the last code was sent less than 30 seconds ago, return TOO_MANY_REQUESTS
    // Since it is ordered by created_at DESC, the first one is the most recent
    if let Some(code) = currently_valid_codes.first() {
        let created_at: DateTime<Utc> = code.created_at;
        let now = Utc::now();
        let duration = now - created_at;

        if duration.num_seconds() < 30 {
            return StatusCode::TOO_MANY_REQUESTS.into_response();
        }
    }

    // Create a random 6 digit code that is not a duplicate
    let mut email_verification_code = rand::thread_rng().gen_range(100000..999999);

    // Ensure that the code is not a duplicate
    for existing_code in currently_valid_codes.iter() {
        if existing_code.created_at.timestamp() as u64 == email_verification_code {
            email_verification_code = rand::thread_rng().gen_range(100000..999999);
        }
    }

    // Create tera context
    let mut context = tera::Context::new();
    context.insert("code", &email_verification_code);

    // Render the email template
    let email_body = match tera.render("verification_email.html", &context) {
        Ok(email_body) => email_body,
        Err(e) => {
            error!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Create the email
    let email = match Message::builder()
        .from(GLOBAL_CONFIG.from_email.parse().unwrap())
        .to(payload.email.parse().unwrap())
        .subject(GLOBAL_CONFIG.default_subject.clone())
        .header(ContentType::TEXT_HTML)
        .body(email_body)
    {
        Ok(email) => email,
        Err(e) => {
            error!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let creds = Credentials::new(
        GLOBAL_CONFIG.smtp_username.clone(),
        GLOBAL_CONFIG.smtp_password.clone(),
    );

    debug!("creating mailer");

    // Mail
    let mailer = match SmtpTransport::relay(&GLOBAL_CONFIG.smtp_server) {
        Ok(mailer) => mailer.credentials(creds).build(),
        Err(e) => {
            error!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    debug!("Sending email to {}", payload.email);

    // Send the email.
    match mailer.send(&email) {
        Ok(_) => {
            // Success, insert the code into the db
            query!(
                r#"
                INSERT INTO verification_codes (email, code, created_at, expires_at)
                VALUES (?, ?, NOW(), NOW() + INTERVAL 10 MINUTE)
                "#,
                payload.email,
                email_verification_code
            )
            .execute(&pool)
            .await
            .expect("Failed to insert verification code");

            StatusCode::OK.into_response()
        }
        Err(e) => {
            error!("Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
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
pub async fn register_user(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // Check if email is already taken
    let user_exists = query!(
        r#"
        SELECT COUNT(*) as user_count
        FROM users
        WHERE email = ?
        "#,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check if user exists");

    if user_exists.user_count > 0 {
        return StatusCode::BAD_REQUEST.into_response();
    }

    // Check if password is valid using zxcvbn
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    let zxcvbn = zxcvbn::zxcvbn(
        &payload.password,
        &[
            &payload.email,
            &payload.invite_code,
            &payload.email_verification_code,
        ],
    );

    if zxcvbn.score() < zxcvbn::Score::Three {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .warning()
                    .map(|s| s.to_string()),
                suggestions: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .suggestions()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            }),
        )
            .into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response();
    }

    // Check if code is valid
    let code_id = query!(
        r#"
        SELECT id
        FROM verification_codes
        WHERE email = ?
        AND code = ?
        AND is_used = false
        AND expires_at > NOW()
        "#,
        payload.email,
        payload.email_verification_code
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to check if code is valid");

    let code_id = match code_id {
        Some(code_id) => Some(code_id.id),
        None => return StatusCode::FORBIDDEN.into_response(),
    };

    // All checks passed, register the user and mark the verification code as used

    // Mark the code as used
    query!(
        r#"
        UPDATE verification_codes
        SET is_used = true
        WHERE id = ?
        "#,
        code_id
    )
    .execute(&pool)
    .await
    .expect("Failed to mark code as used");

    // Hash the password
    let password_hash = tokio::task::spawn_blocking(move || {
        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(&mut rand::thread_rng());
        argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    })
    .await
    .expect("Failed to hash password in thread");

    // Register the user
    query!(
        r#"
        INSERT INTO users (email, password_hash, created_at, updated_at)
        VALUES (?, ?, NOW(), NOW())
        "#,
        payload.email,
        password_hash
    )
    .execute(&pool)
    .await
    .expect("Failed to insert user into db");

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
pub async fn forgot_password(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> impl IntoResponse {
    // Check if user exists
    let user_exists = query!(
        r#"
        SELECT COUNT(*) as user_count
        FROM users
        WHERE email = ?
        "#,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check if user exists");

    if user_exists.user_count == 0 {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Check if password is valid using zxcvbn
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    let zxcvbn = zxcvbn::zxcvbn(
        &payload.password,
        &[&payload.email, &payload.email_verification_code],
    );

    if zxcvbn.score() < zxcvbn::Score::Three {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .warning()
                    .map(|s| s.to_string()),
                suggestions: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .suggestions()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            }),
        )
            .into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response();
    }

    // Check if code is valid
    let code_id = query!(
        r#"
        SELECT id
        FROM verification_codes
        WHERE email = ?
        AND code = ?
        AND is_used = false
        AND expires_at > NOW()
        "#,
        payload.email,
        payload.email_verification_code
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to check if code is valid");

    let code_id = match code_id {
        Some(code_id) => Some(code_id.id),
        None => return StatusCode::FORBIDDEN.into_response(),
    };

    // All checks passed, register the user and mark the verification code as used

    // Mark the code as used
    query!(
        r#"
        UPDATE verification_codes
        SET is_used = true
        WHERE id = ?
        "#,
        code_id
    )
    .execute(&pool)
    .await
    .expect("Failed to mark code as used");

    // Hash the password
    let password_hash = tokio::task::spawn_blocking(move || {
        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(&mut rand::thread_rng());
        argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    })
    .await
    .expect("Failed to hash password in thread");

    // Update the user's password
    query!(
        r#"
        UPDATE users
        SET password_hash = ?
        WHERE email = ?
        "#,
        password_hash,
        payload.email
    )
    .execute(&pool)
    .await
    .expect("Failed to update password");

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
pub async fn get_documentation(
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
pub async fn list_documentation_options(
    Extension(docs): Extension<SharedDocs>,
) -> impl IntoResponse {
    let docs = docs.read().await;

    let mut os_list: Vec<String> = docs.keys().cloned().collect();

    os_list.sort();

    let response = json!({
        "osList": os_list
    });

    Json(response)
}

/// Handler for the GET '/documentation/categories' route.
pub async fn get_documentation_categories(
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
pub async fn transactions(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Get the user's ID from the session
    let user_id = auth_session.user.unwrap().id;

    // Get the user's transactions from the db
    let transactions = query!(
        r#"
        SELECT
            id as `id: i64`,
            user_id as `user_id: i64`,
            plan_id as `plan_id: i64`,
            amount as `amount: BigDecimal`,
            status as `status: UserTransactionStatusEnum`,
            created_at as `created_at: DateTime<Utc>`,
            updated_at as `updated_at: DateTime<Utc>`
        FROM transactions
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch transactions")
    .iter_mut()
    .map(|x| UserTransactionRust {
        id: x.id,
        user_id: x.user_id,
        plan_id: x.plan_id,
        amount: x
            .amount
            .to_f64()
            .expect("Failed to convert BigDecimal to f64")
            / 100.0,
        status: x.status.clone(),
        // stripe_payment_intent_id: x.stripe_payment_intent_id.clone(),
        created_at: x.created_at.timestamp() as u64,
        updated_at: x.updated_at.timestamp() as u64,
    })
    .collect::<Vec<UserTransactionRust>>();

    Json(transactions).into_response()
}

/// Handler for the GET '/transaction/:id' route.
/// This handler will return Json(UserTransactionPayload)
/// This handler will return the transaction with the given id.
/// This handler requires authentication (managed by axum_login).
pub async fn transactions_id(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Get the transaction from the db by id and user_id
    let user = auth_session.user.unwrap();

    let transaction = query!(
        r#"
        SELECT
            id as `id: i64`,
            user_id as `user_id: i64`,
            plan_id as `plan_id: i64`,
            amount as `amount: BigDecimal`,
            status as `status: UserTransactionStatusEnum`,
            created_at as `created_at: DateTime<Utc>`,
            updated_at as `updated_at: DateTime<Utc>`
        FROM transactions
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch transaction");

    // If there are none, return NOT_FOUND
    let transaction = match transaction {
        Some(transaction) => transaction,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // If the transaction does not belong to the user, return FORBIDDEN
    if transaction.user_id != user.id {
        return StatusCode::FORBIDDEN.into_response();
    }

    Json(UserTransactionRust {
        id: transaction.id,
        user_id: transaction.user_id,
        plan_id: transaction.plan_id,
        amount: transaction
            .amount
            .to_f64()
            .expect("Failed to convert BigDecimal to f64")
            / 100.0,
        status: transaction.status,
        // stripe_payment_intent_id: transaction.stripe_payment_intent_id,
        created_at: transaction.created_at.timestamp() as u64,
        updated_at: transaction.updated_at.timestamp() as u64,
    })
    .into_response()
}

/// Handler for the GET '/transaction/:id/secret' route.
/// This handler will return Json(String)
/// This handler will return the client secret for the transaction with the given id.
/// This handler requires authentication (managed by axum_login).
pub async fn transaction_secret(
    Extension(pool): Extension<MySqlPool>,
    Extension(stripe_client): Extension<stripe::Client>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Get the transaction from the db by id and user_id
    let user = auth_session.user.unwrap();

    let transaction = query!(
        r#"
        SELECT
            id as `id: i64`,
            user_id as `user_id: i64`,
            stripe_payment_intent_id as `stripe_payment_intent_id: String`
        FROM transactions
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch transaction");

    // If there are none, return NOT_FOUND
    let transaction = match transaction {
        Some(transaction) => transaction,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // If the transaction does not belong to the user, return FORBIDDEN
    if transaction.user_id != user.id {
        return StatusCode::FORBIDDEN.into_response();
    }

    let payment_intent = match transaction.stripe_payment_intent_id {
        Some(payment_intent) => payment_intent,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // Get the payment intent
    let payment_intent_id = stripe::PaymentIntentId::from_str(&payment_intent)
        .expect("Failed to parse payment intent id");
    let payment_intent = PaymentIntent::retrieve(&stripe_client, &payment_intent_id, &[])
        .await
        .expect("Failed to retrieve payment intent");

    Json(payment_intent.client_secret).into_response()
}

/// Handler for the POST '/transaction/:id/cancel' route.
/// This handler will cancel the transaction with the given id.
/// This is to be used if the user wants to cancel the transaction.
/// This handler requires authentication (managed by axum_login).
pub async fn transaction_cancel(
    Extension(pool): Extension<MySqlPool>,
    Extension(stripe_client): Extension<stripe::Client>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    // Ensure the user_id in the transaction matches user.id
    let transaction = query!(
        r#"
        SELECT
            id as `id: i64`,
            user_id as `user_id: i64`,
            stripe_payment_intent_id as `stripe_payment_intent_id: String`
        FROM transactions
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch transaction");

    let transaction = match transaction {
        Some(transaction) => transaction,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    if transaction.user_id != user.id {
        return StatusCode::FORBIDDEN.into_response();
    }

    let stripe_payment_intent_id = match transaction.stripe_payment_intent_id {
        Some(payment_intent) => payment_intent,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // Use Stripe client to cancel the payment intent
    let payment_intent_id = stripe::PaymentIntentId::from_str(&stripe_payment_intent_id)
        .expect("Failed to parse payment intent id");

    // Send the request, then let stripe respond to the backend's webhook to update the db.
    PaymentIntent::cancel(
        &stripe_client,
        &payment_intent_id,
        CancelPaymentIntent {
            cancellation_reason: Some(stripe::PaymentIntentCancellationReason::RequestedByCustomer),
        },
    )
    .await
    .expect("Failed to cancel payment intent");

    // Mark the transaction as cancelled in the db
    query!(
        r#"
        UPDATE transactions
        SET status = ?
        WHERE id = ?
        "#,
        UserTransactionStatusEnum::Canceled,
        id
    )
    .execute(&pool)
    .await
    .expect("Failed to update transaction status");

    StatusCode::OK.into_response()
}

/// Handler for the POST '/transaction/:id/complete' route.
/// This handler will complete the transaction with the given id.
/// This handler requires authentication (managed by axum_login).
pub async fn transaction_complete(Path(_id): Path<u32>) -> impl IntoResponse {
    // Would complete the transaction in the db.
    // Would also complete the stripe payment intent.
    StatusCode::OK.into_response()
}

/// Handler for the GET '/server_status' route.
/// This handler will return Json(Vec<ServerStatusPayload>)
/// This handler will return the status of the servers.
/// This handler requires authentication (managed by axum_login).
pub async fn server_status() -> impl IntoResponse {
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
pub async fn plan_details(
    Extension(marzban_client): Extension<MarzbanAPIClient>,
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Get the user's details from the db
    let user = auth_session.user.unwrap();

    // let marzban_username = match user.marzban_username {
    //     Some(ref username) => username,
    //     // No marzban username, no plan details possible.
    //     None => return Json(()).into_response(),
    // };

    let marzban_username = query!(
        r#"
        SELECT marzban_username
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch marzban username");

    let marzban_username = match marzban_username.marzban_username {
        Some(username) => username,
        None => return Json(()).into_response(),
    };

    // Get the user's plan details from Marzban
    let user = marzban_client
        .get_user(&marzban_username)
        .await
        .expect("Failed to get user");

    // If the plan has been expired for more than 14 days, assume the plan doesn't exist.
    // If expire is None, the expiration date is 'never'.
    if let Some(expire) = user.expire {
        debug!("Expire: {}", expire);
        if expire < (chrono::Utc::now() - chrono::Duration::days(14)).timestamp() as u64 {
            debug!("Plan expired more than 14 days ago");
            return Json(()).into_response();
        }
    }

    let status = match user.status {
        UserStatus::Active => PlanStatusEnum::Active,
        UserStatus::Disabled => PlanStatusEnum::Disabled,
        UserStatus::Limited => PlanStatusEnum::Limited,
        UserStatus::Expired => PlanStatusEnum::Expired,
        UserStatus::OnHold => PlanStatusEnum::OnHold,
    };

    let plan_details = PlanDetailsRust {
        expiration: user.expire,
        status,
        data_used: user.used_traffic,
        data_limit: user.data_limit,
    };

    Json(plan_details).into_response()
}

/// Handler for the GET '/plans' route.
/// This handler will return Json(Vec<PlanDetailsPayload>)
/// This handler will return all the plans available.
/// This handler requires authentication (managed by axum_login).
/// Note: All prices are in AUD.
pub async fn plans(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    // Get plans from db
    let plans = query!(
        r#"
        SELECT
            id,
            name,
            price,
            data_limit,
            duration_days,
            description
        FROM plans
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch plans");

    let plans = plans
        .into_iter()
        .map(|plan| PlanPayload {
            id: (plan.id as u32).into(),
            name: plan.name,
            price: plan
                .price
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            data_limit: if plan.data_limit == 0 {
                None
            } else {
                Some(plan.data_limit as f64)
            },
            duration_days: (plan.duration_days as u32).into(),
            description: plan.description,
        })
        .collect::<Vec<PlanPayload>>();

    Json(plans).into_response()
}

/// Handler for the GET '/plans/:id' route.
/// This handler will return Json(PlanPayload)
/// This handler will return the details of the plan with the given id.
/// This handler requires authentication (managed by axum_login).
pub async fn plans_id(
    Extension(pool): Extension<MySqlPool>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Get plan from db
    let plan = query!(
        r#"
        SELECT
            id,
            name,
            price,
            data_limit,
            duration_days,
            description
        FROM plans
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await;

    // If there are none, return NOT_FOUND
    let plan = match plan {
        Ok(plan) => plan,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    Json(PlanPayload {
        id: (plan.id as u32).into(),
        name: plan.name,
        price: plan
            .price
            .to_f64()
            .expect("Failed to convert BigDecimal to f64"),
        data_limit: if plan.data_limit == 0 {
            None
        } else {
            Some(plan.data_limit as f64)
        },
        duration_days: (plan.duration_days as u32).into(),
        description: plan.description,
    })
    .into_response()
}

/// Handler for the POST '/orders' route.
/// This handler will create an order for the user.
/// This handler will return Json(CreateOrderResponsePayload)
/// This handler requires authentication (managed by axum_login).
pub async fn create_transaction(
    Extension(pool): Extension<MySqlPool>,
    Extension(stripe_client): Extension<stripe::Client>,
    auth_session: AuthSession,
    Json(payload): Json<CreateOrderPayload>,
) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    // Get the cost of the plan
    let id: u64 = payload.plan_id.into();
    let plan = query!(
        r#"
        SELECT
            price,
            name as `name: String`
        FROM plans
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch plan");

    let plan = match plan {
        Some(plan) => plan,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    // If there are none, return NOT_FOUND
    let plan_price: i64 = (plan.price * 100_i32)
        .to_i64()
        .expect("Failed to convert BigDecimal to i64");

    let plan_name = plan.name;

    // Create a payment intent
    let mut payment_intent = CreatePaymentIntent::new(plan_price, stripe::Currency::AUD);
    payment_intent.statement_descriptor_suffix = Some("Payment for HiddN Plan");
    payment_intent.receipt_email = Some(user.email.as_str());
    payment_intent.metadata = Some(
        [
            ("plan_id".to_string(), id.to_string()),
            ("plan_name".to_string(), plan_name),
            ("user_id".to_string(), user.id.to_string()),
            ("email".to_string(), user.email.clone()),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    let payment_intent = PaymentIntent::create(&stripe_client, payment_intent)
        .await
        .expect("Failed to create payment intent");

    let payment_intent_status: UserTransactionStatusEnum = payment_intent.status.into();

    // Add the order to the db
    let id = query!(
        r#"
        INSERT INTO transactions (user_id, plan_id, amount, status, stripe_payment_intent_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, NOW(), NOW())
        "#,
        user.id,
        id,
        plan_price,
        payment_intent_status,
        payment_intent.id.to_string()
    ).execute(&pool).await.expect("Failed to insert transaction into db").last_insert_id();

    // Return the payment intent client secret
    Json(CreateOrderResponsePayload {
        order_id: id as u32,
        payment_intent_client_secret: payment_intent
            .client_secret
            .expect("Failed to get client secret"),
    })
    .into_response()
}

/// Handler for the POST '/reset_subscription_url' route.
/// This handler will reset the subscription URL.
/// This handler requires authentication (managed by axum_login).
pub async fn reset_subscription_url(
    Extension(marzban_client): Extension<MarzbanAPIClient>,
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Get the user's marzban username
    let user = auth_session.user.unwrap();

    let record = query!(
        r#"
        SELECT marzban_username
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch marzban username");

    let record = match record {
        Some(username) => username,
        None => return StatusCode::OK.into_response(), // just return OK if no marzban username
    };

    let marzban_username = match record.marzban_username {
        Some(username) => username,
        None => return StatusCode::OK.into_response(), // just return OK if no marzban username
    };

    // Reset the subscription URL
    marzban_client
        .revoke_user_subscription(&marzban_username)
        .await
        .expect("Failed to reset subscription URL");

    StatusCode::OK.into_response()
}

/// Handler for the POST '/update_settings' route.
/// This handler will update the user's settings.
/// This handler requires authentication (managed by axum_login).
pub async fn update_settings(
    auth_session: AuthSession,
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<UserProfileSettingsChangePayload>,
) -> impl IntoResponse {
    let user_id = auth_session.user.unwrap().id;

    if payload.email_data_reminder.is_some() {
        query!(
            r#"
            UPDATE users
            SET email_data_reminder = ?
            WHERE id = ?
            "#,
            payload.email_data_reminder.unwrap(),
            user_id
        )
        .execute(&pool)
        .await
        .expect("Failed to update email_data_reminder");
    }

    if payload.email_expiration_reminder.is_some() {
        query!(
            r#"
            UPDATE users
            SET email_expiration_reminder = ?
            WHERE id = ?
            "#,
            payload.email_expiration_reminder.unwrap(),
            user_id
        )
        .execute(&pool)
        .await
        .expect("Failed to update email_data_reminder");
    }

    StatusCode::OK.into_response()
}

/// Handler for the DELETE '/delete_account' route.
/// This is destructive and should be used with caution.
/// This handler will delete the user's account.
/// This handler requires authentication (managed by axum_login).
pub async fn delete_account(
    auth_session: AuthSession,
    Extension(pool): Extension<MySqlPool>,
    Extension(marzban_client): Extension<MarzbanAPIClient>,
) -> impl IntoResponse {
    // Get the user's ID from the session
    let user_id = auth_session.user.unwrap().id;

    // Get the user's marzban username
    let marzban_username = query!(
        r#"
        SELECT marzban_username as `marzban_username: String`
        FROM users
        WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch marzban username");

    if let Some(marzban_username) = marzban_username.marzban_username {
        // Delete the user from Marzban
        marzban_client
            .delete_user(&marzban_username)
            .await
            .expect("Failed to delete user from Marzban");
    }

    // Delete the user from the db
    query!(
        r#"
        DELETE FROM users
        WHERE id = ?
        "#,
        user_id
    )
    .execute(&pool)
    .await
    .expect("Failed to delete user from db");

    StatusCode::OK.into_response()
}

/// Handler for the POST '/change_password' route.
/// This handler will change the user's password.
/// This handler requires authentication (managed by axum_login).
pub async fn change_password(
    auth_session: AuthSession,
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<ChangePasswordPayload>,
) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    // Check if password is valid using zxcvbn
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    let zxcvbn = zxcvbn::zxcvbn(&payload.new_password, &[&user.email]);

    if zxcvbn.score() < zxcvbn::Score::Three {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .warning()
                    .map(|s| s.to_string()),
                suggestions: zxcvbn
                    .feedback()
                    .expect("should not be None")
                    .suggestions()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            }),
        )
            .into_response();
    }

    // Check if password and confirm password match
    if payload.new_password != payload.confirm_password {
        return (
            StatusCode::CONFLICT,
            Json(PasswordFeedbackPayload {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response();
    }

    // Hash the old password
    let argon2 = argon2::Argon2::default();

    let password_hash = user.password_hash().to_string();

    let validation = tokio::task::spawn_blocking(move || {
        // Verify password
        let password_hash =
            PasswordHash::new(&password_hash).expect("Failed to decode password hash from user db");

        argon2
            .verify_password(payload.old_password.as_bytes(), &password_hash)
            .is_ok()
    })
    .await
    .expect("Failed to hash password in thread");

    if !validation {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Verify password
    let password_hash = tokio::task::spawn_blocking(move || {
        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(&mut rand::thread_rng());
        argon2
            .hash_password(payload.new_password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    })
    .await
    .expect("Failed to hash password in thread");

    // Update the user's password
    query!(
        r#"
        UPDATE users
        SET password_hash = ?
        WHERE id = ?
        "#,
        password_hash,
        user.id
    )
    .execute(&pool)
    .await
    .expect("Failed to update password");

    StatusCode::OK.into_response()
}

/// Handler for the GET '/announcements' route.
/// This handler will return Json(Vec<AnnouncementPayload>)
/// This handler will return the announcements.
/// This handler requires authentication (managed by axum_login).
pub async fn get_announcements(
    Extension(announcements): Extension<Arc<RwLock<Vec<AnnouncementPayload>>>>,
) -> impl IntoResponse {
    let announcements = announcements.read().await;

    Json(announcements.clone()).into_response()
}

/// Handler for the GET '/me' route.
/// This handler will return Json(UserProfilePayload)
/// This handler will return the user's profile.
/// This handler requires authentication (managed by axum_login).
pub async fn user_me(
    auth_session: AuthSession,
    Extension(pool): Extension<MySqlPool>,
) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    let user_profile = query!(
        r#"
        SELECT
            email,
            created_at as `created_at: DateTime<Utc>`,
            updated_at as `updated_at: DateTime<Utc>`,
            email_expiration_reminder as `email_expiration_reminder: bool`,
            email_data_reminder as `email_data_reminder: bool`
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch user profile");

    let user_profile = UserProfileRust {
        email: user.email,
        created_at: user_profile.created_at.timestamp() as u64,
        updated_at: user_profile.updated_at.timestamp() as u64,
        email_expiration_reminder: user_profile.email_expiration_reminder,
        email_data_reminder: user_profile.email_data_reminder,
    };

    Json(user_profile).into_response()
}

/// Handler for the GET '/hiddnet_config' route.
/// Returns Clash Meta `config.yaml` for HiddNet use.
/// This handler requires authentication (managed by axum_login).
/// Note: For HiddNet, please ensure that tun[enabled] is set to false initially to
/// prevent the user from connecting to the VPN before the user has paid.
pub async fn hiddnet_config(
    Extension(marzban_client): Extension<MarzbanAPIClient>,
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Get the user's marzban username
    let user = auth_session.user.unwrap();

    let record = query!(
        r#"
        SELECT marzban_username
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch marzban username");

    let record = match record {
        Some(username) => username,
        None => return StatusCode::NOT_FOUND.into_response(), // just return NOT_FOUND if no marzban username
    };

    let marzban_username = match record.marzban_username {
        Some(username) => username,
        None => return StatusCode::NOT_FOUND.into_response(), // just return NOT_FOUND if no marzban username
    };

    // Fetch the user's configuration from Marzban
    let user_info = marzban_client
        .get_user(&marzban_username)
        .await
        .expect("Failed to get user");

    let subscription_url = user_info.subscription_url + "/clash-meta";

    // Go to the url and get the configuration

    let stream = reqwest::get(subscription_url).await.map_err(|e| {
        error!("Failed to get clash meta: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    let stream = match stream {
        Ok(stream) => stream.bytes_stream(),
        Err(status) => return status.into_response(),
    };

    Body::from_stream(stream).into_response()
}

pub struct StripeEvent(stripe::Event);

#[async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = match req.headers().get("stripe-signature") {
            Some(sig) => sig.to_owned(),
            _ => {
                error!("Missing stripe-signature header");
                return Err(StatusCode::BAD_REQUEST.into_response());
            }
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(
                &payload,
                signature.to_str().unwrap(),
                GLOBAL_CONFIG.stripe_webhook_secret.as_str(),
            )
            .map_err(|_| {
                error!("Failed to construct stripe event");
                StatusCode::BAD_REQUEST.into_response()
            })?,
        ))
    }
}

/// Handler for the POST '/stripe' route.
/// This handler will handle the stripe webhook.
pub async fn stripe_webhook(
    Extension(pool): Extension<MySqlPool>,
    Extension(marzban_client): Extension<MarzbanAPIClient>,
    StripeEvent(event): StripeEvent,
) {
    debug!("Received stripe event: {:?}", event);
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            debug!("PaymentIntentSucceeded event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();
                let user_id = payment_intent
                    .metadata
                    .get("user_id")
                    .expect("Failed to get user_id from metadata")
                    .parse::<u64>()
                    .expect("Failed to parse user_id from metadata");

                let plan_id = payment_intent
                    .metadata
                    .get("plan_id")
                    .expect("Failed to get plan_id from metadata")
                    .parse::<u64>()
                    .expect("Failed to parse plan_id from metadata");

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::Succeeded,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");

                // Check if user has a Marzban username
                let user = query!(
                    r#"
                    SELECT
                        marzban_username,
                        email
                    FROM users
                    WHERE id = ?
                    "#,
                    user_id
                )
                .fetch_one(&pool)
                .await
                .expect("Failed to get marzban username");

                // Get list of inbounds available
                let inbounds = marzban_client
                    .get_inbounds()
                    .await
                    .expect("Failed to get inbounds");

                let inbounds = Inbounds {
                    trojan: if inbounds
                        .get(&ProxyTypes::Trojan)
                        .is_none_or(|x| x.is_empty())
                    {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Trojan]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    vless: if inbounds
                        .get(&ProxyTypes::Vless)
                        .is_none_or(|x| x.is_empty())
                    {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Vless]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    vmess: if inbounds
                        .get(&ProxyTypes::Vmess)
                        .is_none_or(|x| x.is_empty())
                    {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Vmess]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    shadowsocks: if inbounds
                        .get(&ProxyTypes::ShadowSocks)
                        .is_none_or(|x| x.is_empty())
                    {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::ShadowSocks]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                };

                let proxies = Proxies {
                    trojan: if inbounds.trojan.is_none() {
                        None
                    } else {
                        Some(Trojan {
                            password: None,
                            flow: None,
                        })
                    },
                    vless: if inbounds.vless.is_none() {
                        None
                    } else {
                        Some(Vless {
                            id: None,
                            flow: Some("xtls-rprx-vision".to_string()),
                        })
                    },
                    vmess: if inbounds.vmess.is_none() {
                        None
                    } else {
                        Some(Vmess {
                            id: None,
                            security: None,
                        })
                    },
                    shadowsocks: if inbounds.shadowsocks.is_none() {
                        None
                    } else {
                        Some(Shadowsocks {
                            password: None,
                            method: None,
                        })
                    },
                };

                let plan = query!(
                    r#"
                    SELECT
                        name,
                        data_limit,
                        duration_days
                    FROM plans
                    WHERE id = ?
                    "#,
                    plan_id
                )
                .fetch_one(&pool)
                .await
                .expect("Failed to get plan");

                // If not, create a user
                match user.marzban_username {
                    None => {
                        // let res = serde_json::ser::to_string(&UserCreate {
                        //     proxies,
                        //     expire: {
                        //         match plan.duration_days {
                        //             0 => None,
                        //             _ => Some(
                        //                 chrono::Utc::now()
                        //                     .checked_add_signed(chrono::Duration::days(
                        //                         plan.duration_days as i64,
                        //                     ))
                        //                     .expect("Failed to add days")
                        //                     .timestamp()
                        //                     as u64,
                        //             ),
                        //         }
                        //     },
                        //     data_limit: {
                        //         // Convert data limit to bytes (where kb = 1024 bytes)
                        //         plan.data_limit as u64 * 1024 * 1024 * 1024
                        //     },
                        //     data_limit_reset_strategy:
                        //         marzban_api::models::user::UserDataLimitResetStrategy::NoReset,
                        //     inbounds,
                        //     note: Some(format!("Created by HiddN. ID: {}", user_id)),
                        //     sub_updated_at: None,
                        //     sub_last_user_agent: None,
                        //     online_at: None,
                        //     on_hold_expire_duration: None,
                        //     on_hold_timeout: None,
                        //     auto_delete_in_days: None,
                        //     username: user.email.clone(),
                        //     status: UserStatusCreate::Active,
                        // });

                        // warn!("UserCreate: {:?}", res);

                        marzban_client
                            .add_user(&UserCreate {
                                proxies,
                                expire: {
                                    match plan.duration_days {
                                        0 => None,
                                        _ => Some(
                                            chrono::Utc::now()
                                                .checked_add_signed(chrono::Duration::days(
                                                    plan.duration_days as i64,
                                                ))
                                                .expect("Failed to add days")
                                                .timestamp()
                                                as u64,
                                        ),
                                    }
                                },
                                data_limit: {
                                    // Convert data limit to bytes (where kb = 1024 bytes)
                                    plan.data_limit as u64 * 1024 * 1024 * 1024
                                },
                                data_limit_reset_strategy:
                                    marzban_api::models::user::UserDataLimitResetStrategy::NoReset,
                                inbounds,
                                note: Some(format!("Created by HiddN. ID: {}", user_id)),
                                sub_updated_at: None,
                                sub_last_user_agent: None,
                                online_at: None,
                                on_hold_expire_duration: None,
                                on_hold_timeout: None,
                                auto_delete_in_days: None,
                                username: user.email.clone(),
                                status: UserStatusCreate::Active,
                            })
                            .await
                            .expect("Failed to create user");

                        // Also add the marzban username to the user in db
                        query!(
                            r#"UPDATE users SET marzban_username = ? WHERE id = ?"#,
                            user.email.clone(),
                            user_id
                        )
                        .execute(&pool)
                        .await
                        .expect("Failed to update marzban username");
                    }
                    Some(marzban_username) => {
                        // Get the user's current plan
                        let current_plan = marzban_client
                            .get_user(&marzban_username)
                            .await
                            .expect("Failed to get user");

                        // If user's expiration is 'never', keep it at 'never'.
                        // If user's expiration is in the past OR data limit has been reached, set it to the new plan's expiration
                        // If user's expiration is in the future, add the new plan's duration to it

                        // If user's data limit is 0, keep it as 0
                        // If user's data limit is not 0 and limit has not been reached, add the new plan's data limit to it
                        // If user's data limit is not 0 and limit has been reached, set it to the new plan's data limit

                        let mut reset_data_usage = false;

                        let new_data_limit = match current_plan.data_limit {
                            None => 0,
                            Some(data_limit) if current_plan.used_traffic >= data_limit => {
                                // If data limit has been reached, set it to the new plan's data limit
                                reset_data_usage = true;
                                plan.data_limit as u64 * 1024 * 1024 * 1024
                            }
                            Some(data_limit) => {
                                // If data limit has not been reached, add the new plan's data limit to it
                                data_limit + plan.data_limit as u64 * 1024 * 1024 * 1024
                            }
                        };

                        let new_expiration = match current_plan.expire {
                            // If expiration is 'never', keep it as 'never'
                            None => 0,
                            Some(expire) => {
                                if expire < chrono::Utc::now().timestamp() as u64
                                    || reset_data_usage
                                {
                                    // If expiration is in the past, set it to the new plan's expiration
                                    // Or, if data limit has been reached, set it to the new plan's expiration
                                    chrono::Utc::now()
                                        .checked_add_signed(chrono::Duration::days(
                                            plan.duration_days as i64,
                                        ))
                                        .expect("Failed to add days")
                                        .timestamp() as u64
                                } else {
                                    // If expiration is in the future, add the new plan's duration to it
                                    chrono::DateTime::from_timestamp(expire as i64, 0)
                                        .expect("Failed to convert to chrono")
                                        .checked_add_signed(chrono::Duration::days(
                                            plan.duration_days as i64,
                                        ))
                                        .expect("Failed to add days")
                                        .timestamp() as u64
                                }
                            }
                        };

                        // Update the user's plan
                        marzban_client.modify_user(&marzban_username, &UserModify {
                            proxies,
                            expire: Some(new_expiration),
                            data_limit: new_data_limit,
                            data_limit_reset_strategy:
                                marzban_api::models::user::UserDataLimitResetStrategy::NoReset,
                            inbounds,
                            note: Some(format!("Updated by HiddN. ID: {}", user_id)),
                            sub_updated_at: None,
                            sub_last_user_agent: None,
                            online_at: None,
                            on_hold_expire_duration: None,
                            on_hold_timeout: None,
                            auto_delete_in_days: None,
                            status: UserStatusModify::Active,
                        }).await.expect("Failed to update user");

                        // If reset_data_usage is true, reset the user's data usage
                        if reset_data_usage {
                            marzban_client
                                .reset_user_data_usage(&marzban_username)
                                .await
                                .expect("Failed to reset user data usage");
                        }
                    }
                };
            }
        }
        EventType::PaymentIntentPaymentFailed => {
            debug!("PaymentIntentPaymentFailed event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::RequiresPaymentMethod,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");
            }
        }
        EventType::PaymentIntentCanceled => {
            debug!("PaymentIntentCanceled event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::Canceled,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");
            }
        }
        EventType::PaymentIntentRequiresAction => {
            debug!("PaymentIntentRequiresAction event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::RequiresAction,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");
            }
        }
        EventType::PaymentIntentRequiresCapture => {
            debug!("PaymentIntentRequiresCapture event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::RequiresCapture,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");
            }
        }
        EventType::PaymentIntentProcessing => {
            debug!("PaymentIntentProcessing event received");
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();

                // Update the transaction status
                query!(
                    r#"
                UPDATE transactions
                SET status = ?
                WHERE stripe_payment_intent_id = ?
                "#,
                    UserTransactionStatusEnum::Processing,
                    payment_intent_id
                )
                .execute(&pool)
                .await
                .expect("Failed to update transaction status");
            }
        }
        _ => {}
    }
}

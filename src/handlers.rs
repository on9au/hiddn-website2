use std::{collections::HashMap, sync::Arc};

use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequest, Request};
use marzban_api::models::proxy::ProxyTypes;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;

use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use marzban_api::client::MarzbanAPIClient;
use marzban_api::models::user::{
    Inbounds, Proxies, Shadowsocks, Trojan, UserCreate, UserModify, UserStatus, UserStatusCreate,
    UserStatusModify, Vless, Vmess,
};
use num_traits::ToPrimitive;
use serde_json::json;
use sqlx::types::BigDecimal;
use sqlx::{query, MySqlPool};
use stripe::{CreatePaymentIntent, EventObject, EventType, PaymentIntent};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

use crate::payloads::{
    ChangePasswordPayload, CreateOrderPayload, PlanDetailsRust, UserProfileSettingsChangePayload,
    UserTransactionRust,
};
use crate::{
    payloads::{
        AnnouncementPayload, CreateOrderResponsePayload, ForgotPasswordPayload, LoginPayload,
        LoginResponsePayload, PasswordFeedbackPayload, PlanPayload, PlanStatusEnum,
        RegisterPayload, RequestCodePayload, ServerStatusPayload, UserProfilePayload,
        UserTransactionStatusEnum, VerifyEmailPayload,
    },
    sessions::AuthSession,
    SharedDocs,
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
pub async fn verify_email(Json(_payload): Json<VerifyEmailPayload>) -> impl IntoResponse {
    // TODO: Implement actual email verification logic
    // We would create a temporary verificaton code linked to the email in the db which expires after a certain time.
    // We would send the verification code to the email.

    StatusCode::OK.into_response()
}

/// Handler for the POST `/request_code` route.
/// This handler will receive a JSON(RequestCodePayload) payload from the client.
/// It will send code to email to verify the email.
/// Should have a rate limit to prevent spamming.
/// This acts as a way to verify the email's ownership and existence.
pub async fn request_code(Json(_payload): Json<RequestCodePayload>) -> impl IntoResponse {
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

    // Check if code is valid
    // Example code here since email client is not implemented
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
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

    // Check if code is valid
    // Example code here since email client is not implemented
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
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
            .expect("Failed to convert BigDecimal to f64"),
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
            .expect("Failed to convert BigDecimal to f64"),
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
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Get the user's details from the db
    let user = auth_session.user.unwrap();

    let marzban_username = match user.marzban_username {
        Some(ref username) => username,
        // No marzban username, no plan details possible.
        None => return Json(()).into_response(),
    };

    // Get the user's plan details from Marzban
    let user = marzban_client
        .get_user(marzban_username)
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
pub async fn reset_subscription_url() -> impl IntoResponse {
    // Typically, would call marzban api to reset the subscription URL.
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
/// This handler will delete the user's account.
/// This handler requires authentication (managed by axum_login).
pub async fn delete_account() -> impl IntoResponse {
    // Typically, would delete the user's account in the db.
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
pub async fn user_me(auth_session: AuthSession) -> impl IntoResponse {
    let user = auth_session.user.unwrap();

    let user_profile = UserProfilePayload {
        email: user.email,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
        email_expiration_reminder: user.email_expiration_reminder,
        email_data_reminder: user.email_data_reminder,
    };

    Json(user_profile).into_response()
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
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(&payload, signature.to_str().unwrap(), "whsec_xxxxx")
                .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

/// Handler for the POST '/stripe_webhook' route.
/// This handler will handle the stripe webhook.
pub async fn stripe_webhook(
    Extension(pool): Extension<MySqlPool>,
    Extension(marzban_client): Extension<MarzbanAPIClient>,
    StripeEvent(event): StripeEvent,
) {
    info!("Received stripe event: {:?}", event);
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            info!("PaymentIntentSucceeded event received");
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
                    trojan: if inbounds[&ProxyTypes::Trojan].is_empty() {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Trojan]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    vless: if inbounds[&ProxyTypes::Vless].is_empty() {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Trojan]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    vmess: if inbounds[&ProxyTypes::Vmess].is_empty() {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Trojan]
                                .iter()
                                .map(|x| x.tag.clone())
                                .collect(),
                        )
                    },
                    shadowsocks: if inbounds[&ProxyTypes::ShadowSocks].is_empty() {
                        None
                    } else {
                        Some(
                            inbounds[&ProxyTypes::Trojan]
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
                            flow: Some("xtls-rprx-direct".to_string()),
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
                                    plan.data_limit as u64 * 1024
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
            info!("PaymentIntentPaymentFailed event received");
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
            info!("PaymentIntentCanceled event received");
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
            info!("PaymentIntentRequiresAction event received");
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
            info!("PaymentIntentRequiresCapture event received");
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
            info!("PaymentIntentProcessing event received");
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

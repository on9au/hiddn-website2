use std::{collections::HashMap, sync::Arc};

use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};

use axum::response::IntoResponse;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use marzban_api::client::MarzbanAPIClient;
use marzban_api::models::user::UserStatus;
use num_traits::ToPrimitive;
use serde_json::json;
use sqlx::{query, MySqlPool};
use tokio::sync::RwLock;
use tracing::error;

use crate::payloads::{ChangePasswordPayload, PlanDetailsRust, UserProfileSettingsChangePayload};
use crate::{
    payloads::{
        AnnouncementPayload, CreateOrderResponsePayload, ForgotPasswordPayload, LoginPayload,
        LoginResponsePayload, PasswordFeedbackPayload, PlanPayload, PlanStatusEnum,
        RegisterPayload, RequestCodePayload, ServerStatusPayload, UserProfilePayload,
        UserTransactionPayload, UserTransactionStatusEnum, VerifyEmailPayload,
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
pub async fn transactions() -> impl IntoResponse {
    let transactions: Vec<UserTransactionPayload> = vec![
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
pub async fn transactions_id(Path(id): Path<u32>) -> impl IntoResponse {
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
    match user.expire {
        Some(expire) => {
            if expire < (chrono::Utc::now() - chrono::Duration::days(14)).timestamp() as u64 {
                return Json(()).into_response();
            }
        }
        None => return Json(()).into_response(),
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
    // let plans = vec![
    //     PlanPayload {
    //         id: 0_u32.into(),
    //         name: "Basic".to_string(),
    //         price: 5.0,
    //         data_limit: Some(20.0),
    //         duration_days: 30_u32.into(),
    //         description: Some("Basic plan".to_string()),
    //     },
    //     PlanPayload {
    //         id: 1_u32.into(),
    //         name: "Premium".to_string(),
    //         price: 10.0,
    //         data_limit: Some(40.0),
    //         duration_days: 30_u32.into(),
    //         description: Some("Premium plan".to_string()),
    //     },
    // ];

    // Json(plans).into_response()

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
pub async fn plans_id(Path(id): Path<u32>) -> impl IntoResponse {
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
pub async fn create_transaction() -> impl IntoResponse {
    // Would create a new order in the db.
    // Would also create stripe payment intent.
    Json(CreateOrderResponsePayload {
        order_id: 1_u32,
        payment_intent_client_secret: "pi_123456".to_string(),
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
    let user_id = auth_session.user.unwrap().id();

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
        user.id()
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

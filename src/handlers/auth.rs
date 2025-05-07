use crate::{
    payloads::{LoginPayload, LoginResponse, RegisterPayload},
    sessions::AuthSession,
    state::AppState,
};
use axum::{Extension, Json, response::IntoResponse};
use reqwest::StatusCode;
use tracing::error;

/// POST `/api/auth/login`
pub async fn login(
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

    (StatusCode::OK, Json(LoginResponse { logged_in: true })).into_response()
}

/// POST `/api/auth/logout`
pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!("Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// GET `/api/auth/is-logged-in`
pub async fn is_logged_in(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_) => StatusCode::OK.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

/// POST `/api/auth/register`
pub async fn register_user(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // Check if the user already exists
    let user = app_state
        .user_repository()
        .get_user_by_email(&payload.email)
        .await
        .unwrap_or(None);

    Json("stub: register_user")
}

/// POST `/api/auth/forgot-password`
pub async fn forgot_password(/* params */) -> impl IntoResponse {
    // UserRepository::forgot_password(...)
    Json("stub: forgot_password")
}

/// POST `/api/auth/email/request-code`
pub async fn request_code(/* params */) -> impl IntoResponse {
    // UserRepository::request_code(...)
    Json("stub: request_code")
}

/// POST `/api/auth/email/verify`
pub async fn verify_email(/* params */) -> impl IntoResponse {
    // UserRepository::verify_email(...)
    Json("stub: verify_email")
}

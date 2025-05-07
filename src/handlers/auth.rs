use crate::{
    errors::AppResult,
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
) -> AppResult<impl IntoResponse> {
    // Authenticate the user with the db
    let user = match auth_session.authenticate(payload).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(StatusCode::UNAUTHORIZED.into_response()),
        Err(e) => {
            error!("Error: {}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    // Log the user in
    auth_session.login(&user).await?;

    Ok((StatusCode::OK, Json(LoginResponse { logged_in: true })).into_response())
}

/// POST `/api/auth/logout`
pub async fn logout(mut auth_session: AuthSession) -> AppResult<impl IntoResponse> {
    auth_session.logout().await?;
    Ok(StatusCode::OK.into_response())
}

/// GET `/api/auth/is-logged-in`
pub async fn is_logged_in(auth_session: AuthSession) -> AppResult<impl IntoResponse> {
    match auth_session.user {
        Some(_) => Ok(StatusCode::OK.into_response()),
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

/// POST `/api/auth/register`
pub async fn register_user(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    // Check if the user already exists
    if app_state
        .user_repository()
        .user_exists(&payload.email)
        .await?
    {
        return Ok(StatusCode::CONFLICT.into_response());
    }

    Ok(Json("stub: register_user").into_response())
}

/// POST `/api/auth/forgot-password`
pub async fn forgot_password(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::forgot_password(...)
    Ok(Json("stub: forgot_password"))
}

/// POST `/api/auth/email/request-code`
pub async fn request_code(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::request_code(...)
    Ok(Json("stub: request_code"))
}

/// POST `/api/auth/email/verify`
pub async fn verify_email(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::verify_email(...)
    Ok(Json("stub: verify_email"))
}

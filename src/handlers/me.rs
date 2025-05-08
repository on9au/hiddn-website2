use crate::{errors::AppResult, sessions::AuthSession, state::AppState};
use anyhow::Context;
use axum::{Extension, Json, response::IntoResponse};

/// GET `/api/me`
pub async fn get_me(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    let user_profile = app_state
        .user_repository()
        .get_user_by_id(user.id)
        .await
        .context("Failed to get user profile")?;

    let user_profile = user_profile.context("User not found")?;

    Ok(Json(user_profile).into_response())
}

/// POST `/api/me/reset-subscription-url`
pub async fn reset_subscription_url(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::reset_subscription_url(...)
    Ok(Json("stub: reset_subscription_url"))
}

/// POST `/api/me/update-settings`
pub async fn update_settings(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::update_settings(...)
    Ok(Json("stub: update_settings"))
}

/// DELETE `/api/me/delete-account`
pub async fn delete_account(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::delete_account(...)
    Ok(Json("stub: delete_account"))
}

/// POST `/api/me/change-password`
pub async fn change_password(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::change_password(...)
    Ok(Json("stub: change_password"))
}

use crate::{
    errors::AppResult,
    payloads::{ChangePasswordPayload, UserProfileSettingsChange},
    sessions::AuthSession,
    state::AppState,
};
use anyhow::Context;
use axum::{Extension, Json, response::IntoResponse};
use reqwest::StatusCode;

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
        .context("Failed to get user profile")
        .context("User not found")?;

    Ok(Json(user_profile).into_response())
}

/// POST `/api/me/reset-subscription-url`
pub async fn reset_subscription_url(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    let marzban_user = app_state
        .user_repository()
        .get_marzban_username(user.id)
        .await
        .context("Failed to get user's marzban username")?;

    let marzban_user = match marzban_user {
        Some(marzban_user) => marzban_user,
        None => {
            // No Marzban user = No subscription.
            // Just ignore the request.
            return Ok(StatusCode::OK.into_response());
        }
    };

    // Invoke Marzban API to reset the subscription URL
    app_state
        .marzban_client()
        .revoke_user_subscription(&marzban_user)
        .await
        .context("Failed to reset subscription URL.")?;

    Ok(StatusCode::OK.into_response())
}

/// POST `/api/me/update-settings`
pub async fn update_settings(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<UserProfileSettingsChange>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    app_state
        .user_repository()
        .update_settings(user.id, &payload)
        .await
        .context("Failed to update user settings")?;

    Ok(StatusCode::OK.into_response())
}

/// DELETE `/api/me/delete-account`
pub async fn delete_account(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    // Get the Marzban username (so we can delete the Marzban account too)
    let marzban_user = app_state
        .user_repository()
        .get_marzban_username(user.id)
        .await
        .context("Failed to get user's marzban username")?;

    if let Some(marzban_user) = marzban_user {
        // Invoke Marzban API to delete the user
        app_state
            .marzban_client()
            .delete_user(&marzban_user)
            .await
            .context("Failed to delete user from Marzban")?;
    }

    // Delete the user from the database
    app_state
        .user_repository()
        .delete_user(user.id)
        .await
        .context("Failed to delete user")?;

    Ok(StatusCode::OK.into_response())
}

/// POST `/api/me/change-password`
pub async fn change_password(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<ChangePasswordPayload>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    // Check if old password is correct
    let is_valid = app_state
        .user_repository()
        .get_password_hash(user.id)
        .await
        .context("Failed to get password hash")?;

    Ok(Json("stub: change_password"))
}

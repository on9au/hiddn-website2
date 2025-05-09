use crate::{
    errors::AppResult,
    payloads::{ChangePasswordPayload, PasswordFeedback, PlanDetails, UserProfileSettingsChange},
    security,
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

/// GET `/api/me/plan-details`
pub async fn get_plan_details(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    let marzban_username = match app_state
        .user_repository()
        .get_marzban_username(user.id)
        .await
        .context("Failed to get user's marzban username")?
    {
        Some(marzban_username) => marzban_username,
        None => {
            // No Marzban user = No subscription.
            // Just ignore the request.
            return Ok(Json(()).into_response());
        }
    };

    let marzban_user = app_state
        .marzban_client()
        .get_user(&marzban_username)
        .await
        .context("Failed to get Marzban user")?;

    // If the plan has been expired for more than 14 days, assume the plan doesn't exist.
    // If expire is None, the expiration date is 'never'.
    if let Some(expire) = marzban_user.expire {
        if expire < (chrono::Utc::now() - chrono::Duration::days(14)).timestamp() as u64 {
            return Ok(Json(()).into_response());
        }
    }

    Ok(Json(std::convert::Into::<PlanDetails>::into(marzban_user)).into_response())
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

    // Check if the old password is correct
    if !security::verify_password(&payload.old_password, user.password_hash()).await? {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    // Check password strength
    let (score, warning, suggestions) =
        security::check_password_strength(&payload.new_password, &[&user.email]);

    if score < zxcvbn::Score::Three {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning,
                suggestions,
            }),
        )
            .into_response());
    }

    // Check if both passwords match
    if payload.new_password != payload.confirm_password {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response());
    }

    // All checks passed, so we can update the password
    let new_password_hash = security::hash_password(&payload.new_password)
        .await
        .context("Failed to hash new password")?;

    app_state
        .user_repository()
        .update_password(user.id, &new_password_hash)
        .await
        .context("Failed to update password")?;

    Ok(StatusCode::OK.into_response())
}

use crate::{
    errors::AppResult,
    payloads::{AdminUserCreate, AdminUserModify},
    security,
    state::AppState,
};
use anyhow::Context;
use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};

/// GET `/api/admin/users`
pub async fn admin_get_users(
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    // Return ALL users
    let all_users = app_state.user_repository().get_all_users().await?;

    // Return the users as JSON
    Ok(Json(all_users).into_response())
}

/// POST `/api/admin/users`
pub async fn admin_create_user(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<AdminUserCreate>,
) -> AppResult<impl IntoResponse> {
    // Check if the user already exists
    if app_state
        .user_repository()
        .user_exists(&payload.email)
        .await?
    {
        return Ok(StatusCode::CONFLICT.into_response());
    }

    // Hash the password
    let password_hash = security::hash_password(&payload.password).await?;

    // Create the user
    app_state
        .user_repository()
        .create_user(&payload.email, &password_hash, payload.admin)
        .await?;

    // Return the user as JSON
    let user = app_state
        .user_repository()
        .get_user_by_email(&payload.email)
        .await?
        .context("Failed to get user")?;

    Ok(Json(user).into_response())
}

/// GET `/api/admin/users/:id`
pub async fn admin_get_user(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let user = app_state
        .user_repository()
        .get_user_admin(id as i64)
        .await?
        .context("Failed to get user")?;

    Ok(Json(user).into_response())
}

/// PUT `/api/admin/users/:id`
pub async fn admin_update_user(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<AdminUserModify>,
) -> AppResult<impl IntoResponse> {
    // Check if the user exists
    if app_state
        .user_repository()
        .user_exists(&payload.email)
        .await?
    {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    // Update the user
    app_state
        .user_repository()
        .update_user_admin(
            id as i64,
            payload.marzban_username,
            &payload.email,
            payload.admin,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn admin_delete_user(
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    // Delete the user
    if app_state.user_repository().delete_user(id as i64).await? {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

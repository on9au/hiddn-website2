use crate::{errors::AppResult, sessions::AuthSession, state::AppState};
use anyhow::Context;
use axum::{Extension, body::Body, response::IntoResponse};
use reqwest::StatusCode;

pub async fn get_clash_meta_config(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    // Get the Marzban username, so we can get the Clash Meta config
    let marzban_user = app_state
        .user_repository()
        .get_marzban_username(user.id)
        .await
        .context("Failed to get user's marzban username")?;

    if let Some(marzban_user) = marzban_user {
        // Get user information from Marzban
        let user_info = app_state
            .marzban_client()
            .get_user(&marzban_user)
            .await
            .context("Failed to get user info from Marzban")?;

        // Get Sub URL from user info
        let subscription_url = user_info.subscription_url + "/clash-meta";

        // Go to the url and get the configuration
        let stream = reqwest::get(subscription_url)
            .await
            .context("Failed to get clash meta config")?
            .bytes_stream();

        Ok(Body::from_stream(stream).into_response())
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }

    // Ok(Json("stub: get_clash_meta_config").into_response())
}

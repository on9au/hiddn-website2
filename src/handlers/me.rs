use crate::errors::AppResult;
use axum::{Json, response::IntoResponse};

pub async fn get_me(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::get_me(...)
    Ok(Json("stub: get_me"))
}

pub async fn reset_subscription_url(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::reset_subscription_url(...)
    Ok(Json("stub: reset_subscription_url"))
}

pub async fn update_settings(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::update_settings(...)
    Ok(Json("stub: update_settings"))
}

pub async fn delete_account(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::delete_account(...)
    Ok(Json("stub: delete_account"))
}

pub async fn change_password(/* params */) -> AppResult<impl IntoResponse> {
    // UserRepository::change_password(...)
    Ok(Json("stub: change_password"))
}

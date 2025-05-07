use crate::db::UserRepository;
use axum::{Json, response::IntoResponse};

pub async fn get_me(/* params */) -> impl IntoResponse {
    // UserRepository::get_me(...)
    Json("stub: get_me")
}

pub async fn reset_subscription_url(/* params */) -> impl IntoResponse {
    // UserRepository::reset_subscription_url(...)
    Json("stub: reset_subscription_url")
}

pub async fn update_settings(/* params */) -> impl IntoResponse {
    // UserRepository::update_settings(...)
    Json("stub: update_settings")
}

pub async fn delete_account(/* params */) -> impl IntoResponse {
    // UserRepository::delete_account(...)
    Json("stub: delete_account")
}

pub async fn change_password(/* params */) -> impl IntoResponse {
    // UserRepository::change_password(...)
    Json("stub: change_password")
}

use crate::db::UserRepository;
use axum::{Json, response::IntoResponse};

pub async fn login(/* params */) -> impl IntoResponse {
    // UserRepository::login(...)
    Json("stub: login")
}

pub async fn logout(/* params */) -> impl IntoResponse {
    Json("stub: logout")
}

pub async fn is_logged_in(/* params */) -> impl IntoResponse {
    // UserRepository::is_logged_in(...)
    Json("stub: is_logged_in")
}

pub async fn register_user(/* params */) -> impl IntoResponse {
    // UserRepository::register_user(...)
    Json("stub: register_user")
}

pub async fn forgot_password(/* params */) -> impl IntoResponse {
    // UserRepository::forgot_password(...)
    Json("stub: forgot_password")
}

pub async fn request_code(/* params */) -> impl IntoResponse {
    // UserRepository::request_code(...)
    Json("stub: request_code")
}

pub async fn verify_email(/* params */) -> impl IntoResponse {
    // UserRepository::verify_email(...)
    Json("stub: verify_email")
}

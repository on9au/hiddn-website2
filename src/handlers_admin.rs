use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::sessions::AuthSession;

pub async fn admin_me(auth_session: AuthSession) -> impl IntoResponse {
    let user = auth_session.user.unwrap();
    let admin = user.is_admin();

    if admin {
        StatusCode::OK
    } else {
        StatusCode::FORBIDDEN
    }
}

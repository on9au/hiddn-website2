use axum::response::IntoResponse;
use chrono::DateTime;
use reqwest::StatusCode;

use crate::{payloads::AdminUser, sessions::AuthSession};

async fn is_admin(auth_session: &AuthSession) -> bool {
    let user = auth_session.user.as_ref().unwrap();
    user.is_admin()
}

/// /api/admin/me
pub async fn admin_me(auth_session: AuthSession) -> impl IntoResponse {
    if is_admin(&auth_session).await {
        StatusCode::OK
    } else {
        StatusCode::FORBIDDEN
    }
}

/// /api/admin/users
pub async fn admin_users(auth_session: AuthSession) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Fetch all users
    // impl db call here

    // Return all users

    axum::Json(serde_json::json! {
    vec![
        AdminUser {
            id: 0,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
            updated_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
        },
        AdminUser {
            id: 1,
            email: "sett@test.com".to_string(),
            admin: false,
            created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
            updated_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
        },
        AdminUser {
            id: 2,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
            updated_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
        },
        AdminUser {
            id: 3,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
            updated_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
        },
        AdminUser {
            id: 4,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
            updated_at: DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap().into(),
        },
    ]})
    .into_response()
}

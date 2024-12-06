use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use num_traits::FromPrimitive;
use reqwest::StatusCode;
use sqlx::{query, query_as, types::BigDecimal, MySqlPool};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::{
    config::GLOBAL_CONFIG,
    payloads::{
        AdminCreateAnnouncement, AdminUserModify, AdminUserRust, AnnouncementPayload,
        NewPlanPayload, PlanPayload,
    },
    sessions::AuthSession,
};

async fn is_admin(auth_session: &AuthSession) -> bool {
    let user = auth_session.user.as_ref().unwrap();
    user.is_admin()
}

/// GET '/api/admin/me'
pub async fn admin_me(auth_session: AuthSession) -> impl IntoResponse {
    if is_admin(&auth_session).await {
        StatusCode::OK
    } else {
        StatusCode::FORBIDDEN
    }
}

/// GET '/api/admin/my_id'
pub async fn admin_my_id(auth_session: AuthSession) -> impl IntoResponse {
    if is_admin(&auth_session).await {
        axum::Json(auth_session.user.as_ref().unwrap().id).into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

/// POST '/api/admin/announcements'
pub async fn post_announcement(
    auth_session: AuthSession,
    Extension(announcements): Extension<Arc<RwLock<Vec<AnnouncementPayload>>>>,
    Json(new_announcement): Json<AdminCreateAnnouncement>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    let next_id = {
        let announcements = announcements.read().await;
        announcements.len() as u32
    };
    let mut announcements = announcements.write().await;

    // Add to the announcements cache, then the fs
    announcements.push(AnnouncementPayload {
        id: next_id.into(),
        title: new_announcement.title.clone(),
        date: Utc::now().to_rfc3339().to_string(),
        content: new_announcement.content.clone(),
    });

    let file_path = format!(
        "{}/{}.md",
        GLOBAL_CONFIG.announcements_dir, new_announcement.title
    );
    let mut file = File::create(file_path).await.unwrap();
    file.write_all(new_announcement.content.as_bytes())
        .await
        .unwrap();

    // Return the new announcement in full form
    axum::Json(announcements.last().unwrap().clone()).into_response()
}

/// DELETE '/api/admin/announcements/:id'
pub async fn delete_announcement(
    auth_session: AuthSession,
    Extension(announcements): Extension<Arc<RwLock<Vec<AnnouncementPayload>>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    let mut announcements = announcements.write().await;
    let index = announcements
        .iter()
        .position(|a| a.id == typeshare::U53::from(id));
    if let Some(index) = index {
        let name = announcements[index].title.clone();
        announcements.remove(index);
        let file_path = format!("{}/{}.md", GLOBAL_CONFIG.announcements_dir, name);
        tokio::fs::remove_file(file_path).await.unwrap();
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

/// POST '/api/admin/plans'
pub async fn post_plan(
    auth_session: AuthSession,
    Extension(pool): Extension<MySqlPool>,
    Json(new_announcement): Json<NewPlanPayload>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    let duration_days: u64 = new_announcement.duration_days.into();

    // Add the new plan to the database
    let result = query!(
        "INSERT INTO plans (name, price, data_limit, duration_days, description) VALUES (?, ?, ?, ?, ?)",
        new_announcement.name,
        BigDecimal::from_f64(new_announcement.price).expect("Failed to convert price to BigDecimal"),
        (new_announcement.data_limit.unwrap_or(0.0).floor() as u32),
        duration_days,
        new_announcement.description
    ).execute(&pool).await.expect("Failed to insert new plan");

    let plan_id = result.last_insert_id();

    // Return the new plan

    axum::Json(PlanPayload {
        id: (plan_id as u32).into(),
        name: new_announcement.name,
        price: new_announcement.price,
        data_limit: new_announcement.data_limit,
        duration_days: new_announcement.duration_days,
        description: new_announcement.description,
    })
    .into_response()
}

/// DELETE '/api/admin/plans/:id'
pub async fn delete_plan(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Delete the plan from the database
    let result = query!("DELETE FROM plans WHERE id = ?", id)
        .execute(&pool)
        .await;

    // Return the result
    if result.is_err() {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

/// /api/admin/users
pub async fn admin_users(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Fetch all users
    let users = query_as!(
        AdminUserRust,
        r#"
        SELECT 
            id as `id: u32`,
            email,
            marzban_username,
            is_admin as `admin: bool`,
            created_at as `created_at: u64`,
            updated_at as `updated_at: u64`
        FROM users
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch users");

    // Return all users
    axum::Json(users).into_response()
}

/// GET '/api/admin/users/:id'
pub async fn admin_user(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Fetch the user
    let user = query_as!(
        AdminUserRust,
        r#"
        SELECT 
            id as `id: u32`,
            email,
            marzban_username,
            is_admin as `admin: bool`,
            created_at as `created_at: u64`,
            updated_at as `updated_at: u64`
        FROM users
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch user");

    // Return the user
    if let Some(user) = user {
        axum::Json(user).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

/// PUT '/api/admin/users/:id'
pub async fn update_user(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
    Json(payload): Json<AdminUserModify>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Update the user in the database
    let result = query!(
        "UPDATE users SET email = ?, marzban_username = ?, is_admin = ? WHERE id = ?",
        payload.email,
        payload.marzban_username,
        payload.admin,
        id
    )
    .execute(&pool)
    .await;

    // Return the result
    if result.is_err() {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

/// DELETE '/api/admin/users/:id'
pub async fn delete_user(
    Extension(pool): Extension<MySqlPool>,
    auth_session: AuthSession,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Delete the user from the database
    let result = query!("DELETE FROM users WHERE id = ?", id)
        .execute(&pool)
        .await;

    // Return the result
    if result.is_err() {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

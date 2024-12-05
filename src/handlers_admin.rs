use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use chrono::Utc;
use reqwest::StatusCode;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::{
    config::GLOBAL_CONFIG,
    payloads::{
        AdminCreateAnnouncement, AdminUser, AnnouncementPayload, NewPlanPayload, PlanPayload,
    },
    sessions::AuthSession,
};

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
    Json(new_announcement): Json<NewPlanPayload>,
) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // impl db call here

    axum::Json(PlanPayload {
        id: 2_u32.into(),
        name: new_announcement.name,
        price: new_announcement.price,
        data_limit: new_announcement.data_limit,
        duration_days: new_announcement.duration_days,
        description: new_announcement.description,
    })
    .into_response()
}

/// DELETE '/api/admin/plans/:id'
pub async fn delete_plan(auth_session: AuthSession, Path(id): Path<u32>) -> impl IntoResponse {
    // Validate that the user is an admin
    if !is_admin(&auth_session).await {
        return StatusCode::FORBIDDEN.into_response();
    }

    // impl db call here

    StatusCode::NO_CONTENT.into_response()
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
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
        },
        AdminUser {
            id: 1,
            email: "sett@test.com".to_string(),
            admin: false,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
        },
        AdminUser {
            id: 2,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
        },
        AdminUser {
            id: 3,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
        },
        AdminUser {
            id: 4,
            email: "test@test.com".to_string(),
            admin: true,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            updated_at: "2021-01-01T00:00:00Z".to_string(),
        },
    ]})
    .into_response()
}

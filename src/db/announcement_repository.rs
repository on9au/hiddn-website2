//! # Announcement Repository
//!
//! This module contains the `AnnouncementRepository` struct, which is responsible for
//! interacting with the `announcements` table in the database.

use crate::payloads::Announcement;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

/// # Announcement Repository
pub struct AnnouncementRepository {
    pool: MySqlPool,
}

impl AnnouncementRepository {
    /// Creates a new `AnnouncementRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Get all announcements sorted by uid (oldest first).
    pub async fn get_announcements(&self) -> Result<Vec<Announcement>> {
        let announcements = sqlx::query_as!(
            Announcement,
            r#"
            SELECT 
                id,
                title,
                content,
                created_at as `date: DateTime<Utc>`
            FROM announcements
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch announcements")?;

        Ok(announcements)
    }

    /// Get all announcements sorted by date (newest first).
    pub async fn get_announcements_newest(&self) -> Result<Vec<Announcement>> {
        let announcements = sqlx::query_as!(
            Announcement,
            r#"
            SELECT 
                id,
                title,
                content,
                created_at as `date: DateTime<Utc>`
            FROM announcements
            ORDER BY created_at DESC -- newest first
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch announcements")?;

        Ok(announcements)
    }

    /// Get a single announcement by its ID.
    pub async fn get_announcement_by_id(&self, id: i64) -> Result<Option<Announcement>> {
        let announcement = sqlx::query_as!(
            Announcement,
            r#"
            SELECT 
                id,
                title,
                content,
                created_at as `date: DateTime<Utc>`
            FROM announcements
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch announcement")?;

        Ok(announcement)
    }

    /// Create a new announcement.
    pub async fn create_announcement(&self, title: String, content: String) -> Result<()> {
        sqlx::query_as!(
            Announcement,
            r#"
            INSERT INTO announcements (title, content)
            VALUES (?, ?)
            "#,
            title,
            content
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create announcement")?;

        Ok(())
    }

    /// Delete an announcement by its ID.
    pub async fn delete_announcement(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM announcements
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete announcement")?;

        Ok(())
    }
}

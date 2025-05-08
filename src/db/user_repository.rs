use crate::payloads::{UserProfile, UserProfileSettingsChange};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    /// Creates a new `UserRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Get all users (sorted by ID).
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<UserProfile>> {
        let user = sqlx::query_as!(
            UserProfile,
            r#"
            SELECT 
                email,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`,
                email_expiration_reminder as `email_expiration_reminder: bool`,
                email_data_reminder as `email_data_reminder: bool`
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user")?;

        Ok(user)
    }

    /// Get a single user by their email.
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserProfile>> {
        let user = sqlx::query_as!(
            UserProfile,
            r#"
            SELECT 
                email,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`,
                email_expiration_reminder as `email_expiration_reminder: bool`,
                email_data_reminder as `email_data_reminder: bool`
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch user by email")?;

        Ok(user)
    }

    /// Check if a user exists by their email.
    pub async fn user_exists(&self, email: &str) -> Result<bool> {
        let user_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as user_count
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to check if user exists")?
        .user_count;

        Ok(user_count > 0)
    }

    pub async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        is_admin: bool,
    ) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, is_admin, created_at, updated_at)
            VALUES (?, ?, ?, NOW(), NOW())
            "#,
            email,
            password_hash,
            is_admin
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert user into db")?;

        Ok(result.last_insert_id() as i64)
    }

    pub async fn update_password(&self, email: &str, password_hash: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = ?
            WHERE email = ?
            "#,
            password_hash,
            email
        )
        .execute(&self.pool)
        .await
        .context("Failed to update password")?;

        Ok(())
    }

    pub async fn update_settings(
        &self,
        user_id: i64,
        settings: &UserProfileSettingsChange,
    ) -> Result<()> {
        if let Some(email_data_reminder) = settings.email_data_reminder {
            sqlx::query!(
                r#"
                UPDATE users
                SET email_data_reminder = ?
                WHERE id = ?
                "#,
                email_data_reminder,
                user_id
            )
            .execute(&self.pool)
            .await
            .context("Failed to update email_data_reminder")?;
        }

        if let Some(email_expiration_reminder) = settings.email_expiration_reminder {
            sqlx::query!(
                r#"
                UPDATE users
                SET email_expiration_reminder = ?
                WHERE id = ?
                "#,
                email_expiration_reminder,
                user_id
            )
            .execute(&self.pool)
            .await
            .context("Failed to update email_expiration_reminder")?;
        }

        Ok(())
    }

    pub async fn delete_user(&self, id: i64) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .context("Failed to delete user")?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_password_hash(&self, id: i64) -> Result<Option<String>> {
        let result = sqlx::query!(
            r#"
            SELECT password_hash 
            FROM users 
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to get password hash")?;

        Ok(result.map(|row| row.password_hash))
    }

    pub async fn update_marzban_username(&self, user_id: i64, username: &str) -> Result<()> {
        sqlx::query!(
            r#"UPDATE users SET marzban_username = ? WHERE id = ?"#,
            username,
            user_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to update marzban username")?;

        Ok(())
    }

    pub async fn get_marzban_username(&self, user_id: i64) -> Result<Option<String>> {
        let result = sqlx::query!(
            r#"
            SELECT marzban_username
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch marzban username")?;

        Ok(result.and_then(|row| row.marzban_username))
    }
}

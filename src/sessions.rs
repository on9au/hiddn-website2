use argon2::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use sqlx::{query_as, MySqlPool};

use crate::payloads::LoginPayload;

// This is the type we'll use to represent the session for convenience.
pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub marzban_username: Option<String>,
    pub email: String,
    password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email_data_reminder: bool,
    pub email_expiration_reminder: bool,
}

impl User {
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

// To avoid leaking the password in logs, we implement a custom Debug implementation
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("marzban_username", &self.marzban_username)
            .field("email", &self.email)
            .field("password_hash", &"[redacted]")
            .field("password_salt", &"[redacted]")
            .field("is_admin", &self.is_admin)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

// When db implementation is ready, we can implement the AuthnBackend trait for our backend.
// It would look like this:
// pub struct Backend {
//     pool: sqlx::PgPool,
// }
#[derive(Clone)]
pub struct Backend {
    users: MySqlPool,
}

impl Backend {
    pub fn new(pool: MySqlPool) -> Self {
        Self { users: pool }
    }
}

// impl Backend {
//     pub fn new(pool: sqlx::PgPool) -> Self {
//         Self { pool }
//     }

// For credentials, we will use payload::LoginPayload, for simplicity and for typeshare with typescript.

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = LoginPayload;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = query_as!(
            User,
            r#"
            SELECT 
                id,
                marzban_username,
                email,
                password_hash,
                is_admin as `is_admin: bool`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`,
                email_data_reminder as `email_data_reminder: bool`,
                email_expiration_reminder as `email_expiration_reminder: bool`
            FROM users
            WHERE email = ?
            "#,
            creds.email
        )
        .fetch_optional(&self.users)
        .await?;

        if let Some(user) = user {
            // Spawn blocking task to verify password hash
            let password_hash = user.password_hash.clone();
            let validation = tokio::task::spawn_blocking(move || {
                // Verify password
                let argon2 = Argon2::default();

                // We need to convert the password hash from the database to a PasswordHash
                let password_hash = PasswordHash::new(&password_hash)
                    .expect("Failed to decode password hash from user db");

                // Verify the password
                argon2.verify_password(creds.password.as_bytes(), &password_hash)
            })
            .await
            .expect("Failed to verify password hash in thread");

            match validation {
                Ok(()) => Ok(Some(user)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = query_as!(
            User,
            r#"
            SELECT 
                id,
                marzban_username,
                email,
                password_hash,
                is_admin as `is_admin: bool`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`,
                email_data_reminder as `email_data_reminder: bool`,
                email_expiration_reminder as `email_expiration_reminder: bool`
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.users)
        .await?;

        Ok(user)
    }
}

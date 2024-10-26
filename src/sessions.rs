use argon2::{password_hash::SaltString, Argon2, PasswordVerifier};
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::MySqlPool;

use crate::payloads::LoginPayload;

// This is the type we'll use to represent the session for convenience.
pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

// To avoid leaking the password in logs, we implement a custom Debug implementation
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password_hash", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

// DB Backend
#[derive(Clone)]
pub struct Backend {
    pool: MySqlPool,
}

impl Backend {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

// #[derive(Clone, Default)]
// pub struct Backend {
//     users: HashMap<Uuid, User>,
// }

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
        // Fetch user from the database
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, password_salt FROM hiddn_online_users WHERE email = ?",
        )
        .bind(&creds.email)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(user) = user {
            // Verify password
            if verify_password(&creds.password, &user.password_hash)
                .map_err(|e| sqlx::Error::Decode(e.to_string().into()))?
            {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, password_salt FROM hiddn_online_users WHERE id = ?",
        )
        .bind(*user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}

/// Verifies the provided password against the stored hash
fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = argon2::PasswordHash::new(hash)?;

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Hashes the password using Argon2
fn hash_password(password: &str) -> Result<Vec<u8>, argon2::password_hash::Error> {
    let binding = SaltString::generate(&mut rand::thread_rng());
    let salt: &[u8] = binding.as_ref().as_bytes();

    let argon2 = Argon2::default();

    let mut hash = vec![0u8; argon2::Params::DEFAULT_OUTPUT_LEN];

    argon2.hash_password_into(password.as_bytes(), salt, &mut hash)?;

    Ok(hash)
}

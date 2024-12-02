use std::collections::HashMap;

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use uuid::Uuid;

use crate::payloads::LoginPayload;

// This is the type we'll use to represent the session for convenience.
pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Clone)]
pub struct User {
    id: i64,
    pub email: String,
    password_hash: String, // On the DB, it would be salted.
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
#[derive(Clone, Default)]
pub struct Backend {
    #[allow(dead_code)] // We're not using this yet.
    users: HashMap<Uuid, User>,
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
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
        //     .bind(creds.email)
        //     .fetch_optional(&self.db)
        //     .await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        // task::spawn_blocking(|| {
        //     // We're using password-based authentication--this works by comparing our form
        //     // input with an argon2 password hash.
        //     Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        // })
        // .await

        // For now, we'll just return a dummy user.
        if creds.email == "test@test.com" && creds.password == "password" {
            Ok(Some(User {
                id: 1,
                email: "test@test.com".to_string(),
                password_hash: "password".to_string(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        // let user = sqlx::query_as("select * from users where id = ?")
        //     .bind(user_id)
        //     .fetch_optional(&self.db)
        //     .await?;

        // Ok(user)

        if *user_id == 1 {
            Ok(Some(User {
                id: 1,
                email: "test@test.com".to_string(),
                password_hash: "password".to_string(),
            }))
        } else {
            Ok(None)
        }
    }
}

use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

pub struct VerificationRepository {
    pool: MySqlPool,
}

impl VerificationRepository {
    /// Creates a new `VerificationRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Creates a random verification code for the given email.
    ///
    /// This function does not insert the code into the database.
    ///
    /// The verification code is a 6-digit number, and is unique among currently active codes for the given email.
    pub async fn create_random_verification_code(&self, email: &str) -> sqlx::Result<String> {
        let verification_code = loop {
            // Generate a random verification code
            let code: String = (0..6)
                .map(|_| rand::random::<u8>() % 10 + b'0')
                .map(char::from)
                .collect();

            // Check if the code is unique for the given email
            let is_unique = sqlx::query!(
                r#"
                SELECT COUNT(*) as count
                FROM verification_codes
                WHERE email = ? AND code = ? AND expires_at > NOW() AND is_used = 0
                "#,
                email,
                code
            )
            .fetch_one(&self.pool)
            .await?
            .count
                == 0;

            if is_unique {
                break code;
            }
        };

        Ok(verification_code)
    }

    /// Inserts a new verification code into the database.
    /// Ensures that the generated verification code is unique among currently active codes for the given email.
    ///
    /// Returns the verification code and the ID of the inserted verification code.
    pub async fn create_verification_code(&self, email: &str) -> sqlx::Result<(String, u64)> {
        let verification_code = self.create_random_verification_code(email).await?;

        // Insert the unique verification code into the database
        let id = self
            .insert_verification_code(email, &verification_code)
            .await?;

        Ok((verification_code, id))
    }

    /// Inserts a new verification code into the database.
    /// Returns the ID of the inserted verification code.
    pub async fn insert_verification_code(
        &self,
        email: &str,
        verification_code: &str,
    ) -> sqlx::Result<u64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO verification_codes (email, code)
            VALUES (?, ?)
            "#,
            email,
            verification_code
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id())
    }

    /// Checks if the verification code is valid for the given email.
    /// Returns true if the code is valid, false otherwise.
    ///
    /// Will also check if the code is expired, or if the code is used.
    ///
    /// This function will also mark the code as used if it is valid.
    pub async fn is_verification_code_valid(
        &self,
        email: &str,
        verification_code: &str,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT id
            FROM verification_codes
            WHERE email = ? AND code = ? AND expires_at > NOW() AND is_used = 0
            "#,
            email,
            verification_code
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(id) = result {
            // Mark the code as used
            sqlx::query!(
                r#"
                UPDATE verification_codes
                SET is_used = 1
                WHERE id = ?
                "#,
                id.id
            )
            .execute(&self.pool)
            .await?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the timestamp of the last verification code sent to the given email.
    pub async fn get_last_verification_code_timestamp(
        &self,
        email: &str,
    ) -> sqlx::Result<Option<DateTime<Utc>>> {
        let result = sqlx::query!(
            r#"
            SELECT created_at
            FROM verification_codes
            WHERE email = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.created_at))
    }
}

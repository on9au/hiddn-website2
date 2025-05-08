use anyhow::{Context, Result, anyhow};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tokio::task;

/// Hash a password using Argon2 and a random salt (async).
pub async fn hash_password(password: &str) -> Result<String> {
    let password = password.to_owned();
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("Failed to hash password"))?
            .to_string();
        Ok(hash)
    })
    .await
    .context("Failed to join blocking task for hashing")?
}

/// Verify a password against a hash using Argon2 (async).
pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let password = password.to_owned();
    let hash = hash.to_owned();
    task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("Failed to parse password hash"))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    })
    .await
    .context("Failed to join blocking task for verification")?
}

/// Check password strength using zxcvbn. Returns (score, warning, suggestions).
pub fn check_password_strength(
    password: &str,
    user_inputs: &[&str],
) -> (zxcvbn::Score, Option<String>, Vec<String>) {
    let estimate = zxcvbn::zxcvbn(password, user_inputs);
    let warning = estimate
        .feedback()
        .and_then(|f| f.warning())
        .map(|s| s.to_string());
    let suggestions = estimate
        .feedback()
        .map(|f| f.suggestions().iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    (estimate.score(), warning, suggestions)
}

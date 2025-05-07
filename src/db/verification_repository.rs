use sqlx::MySqlPool;

pub struct VerificationRepository {
    pool: MySqlPool,
}

impl VerificationRepository {
    /// Creates a new `VerificationRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

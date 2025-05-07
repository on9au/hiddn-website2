use sqlx::MySqlPool;

pub struct TransactionRepository {
    pool: MySqlPool,
}

impl TransactionRepository {
    /// Creates a new `TransactionRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

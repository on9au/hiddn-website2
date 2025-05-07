use sqlx::MySqlPool;

pub struct TransactionRepository {
    pool: MySqlPool,
}

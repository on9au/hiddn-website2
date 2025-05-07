use sqlx::MySqlPool;

pub struct VerificationRepository {
    pool: MySqlPool,
}

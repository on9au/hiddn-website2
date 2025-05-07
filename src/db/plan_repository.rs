use sqlx::MySqlPool;

pub struct PlanRepository {
    pool: MySqlPool,
}

impl PlanRepository {
    /// Creates a new `PlanRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

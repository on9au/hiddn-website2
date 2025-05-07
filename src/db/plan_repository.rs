use sqlx::MySqlPool;

pub struct PlanRepository {
    pool: MySqlPool,
}

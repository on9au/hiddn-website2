use crate::payloads::plan_payloads::Plan;
use anyhow::Result;
use sqlx::MySqlPool;

pub struct PlanRepository {
    pool: MySqlPool,
}

impl PlanRepository {
    /// Creates a new `PlanRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        Ok(vec![])
    }

    pub async fn get_plan_by_id(&self, _id: u32) -> Result<Option<Plan>> {
        Ok(None)
    }
}

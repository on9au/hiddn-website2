use crate::payloads::NewPlan;
use crate::payloads::plan_payloads::Plan;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use sqlx::types::BigDecimal;
pub struct PlanRepository {
    pool: MySqlPool,
}

impl PlanRepository {
    /// Creates a new `PlanRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        let plans = sqlx::query_as!(
            Plan,
            r#"
            SELECT 
                id as `id: u32`,
                enabled as `enabled: bool`,
                name as `name: String`,
                price as `price: BigDecimal`,
                data_limit as `data_limit: u32`,
                duration_days as `duration_days: u32`,
                description as `description: String`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`
            FROM plans
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(plans)
    }

    pub async fn get_plan_by_id(&self, id: i64) -> Result<Option<Plan>> {
        let plan = sqlx::query_as!(
            Plan,
            r#"
            SELECT 
                id as `id: u32`,
                enabled as `enabled: bool`,
                name as `name: String`,
                price as `price: BigDecimal`,
                data_limit as `data_limit: u32`,
                duration_days as `duration_days: u32`,
                description as `description: String`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`
            FROM plans
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn create_plan(&self, plan: &NewPlan) -> Result<u32> {
        let id = sqlx::query!(
            r#"
            INSERT INTO plans (name, price, data_limit, duration_days, description)
            VALUES (?, ?, ?, ?, ?)
            "#,
            plan.name,
            plan.price,
            plan.data_limit,
            plan.duration_days,
            plan.description
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(id as u32)
    }

    /// Deletes a plan by its ID.
    /// Returns true if the plan was deleted, false if it didn't exist.
    pub async fn delete_plan(&self, id: u32) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM plans
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

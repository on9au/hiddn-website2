use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use ts_rs::TS;

use super::enums;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct UserTransaction {
    pub id: u32,
    pub user_id: u32,
    pub plan_id: u32,
    pub amount: BigDecimal,
    pub status: enums::UserTransactionStatus,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

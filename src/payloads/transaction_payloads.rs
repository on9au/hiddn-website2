use serde::Serialize;
use ts_rs::TS;

use super::enums;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct UserTransaction {
    pub id: u64,
    pub user_id: u64,
    pub plan_id: u64,
    pub amount: f64,
    pub status: enums::UserTransactionStatus,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

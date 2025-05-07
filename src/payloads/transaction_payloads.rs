use serde::Serialize;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct UserTransactionPayload {
    pub id: u64,
    pub user_id: u64,
    pub plan_id: u64,
    pub amount: f64,
    pub status: super::enums::UserTransactionStatusEnum,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

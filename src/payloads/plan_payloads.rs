use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct PlanDetailsPayload {
    pub expiration: Option<u64>,
    pub status: super::enums::PlanStatusEnum,
    #[serde(rename = "dataUsed")]
    pub data_used: u64,
    #[serde(rename = "dataLimit")]
    pub data_limit: Option<u64>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct PlanPayload {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: u64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct NewPlanPayload {
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: u64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateOrderPayload {
    pub plan_id: u64,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct CreateOrderResponsePayload {
    pub order_id: u32,
    pub payment_intent_client_secret: String,
}

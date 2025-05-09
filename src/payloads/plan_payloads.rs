use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use marzban_api::models::user::UserResponse;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct PlanDetails {
    pub expiration: Option<DateTime<Utc>>,
    pub status: super::enums::PlanStatusEnum,
    #[serde(rename = "dataUsed")]
    pub data_used: u64,
    #[serde(rename = "dataLimit")]
    pub data_limit: Option<u64>,
}

impl From<UserResponse> for PlanDetails {
    fn from(user: UserResponse) -> Self {
        Self {
            expiration: {
                user.expire
                    .map(|expire| Utc.timestamp_opt(expire as i64, 0).unwrap())
            },
            status: user.status.into(),
            data_used: user.used_traffic,
            data_limit: user.data_limit,
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct Plan {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: u64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct NewPlan {
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: u64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateOrder {
    pub plan_id: u64,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct CreateOrderResponse {
    pub order_id: u32,
    pub payment_intent_client_secret: String,
}

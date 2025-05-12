use chrono::{DateTime, TimeZone, Utc};
use marzban_api::models::user::UserResponse;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
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
    pub id: u32,
    pub enabled: bool,
    pub name: String,
    pub price: BigDecimal,
    pub data_limit: Option<f64>, // in GB
    pub duration_days: u64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct NewPlan {
    pub name: String,
    pub price: BigDecimal,
    pub data_limit: Option<f64>,
    pub duration_days: u64,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateOrder {
    pub plan_id: u32,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct CreateOrderResponse {
    pub order_id: u32,
    pub payment_intent_client_secret: String,
}

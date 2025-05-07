use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub enum ServerStatusEnum {
    Online,
    Degraded,
    Unreachable,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct ServerStatusPayload {
    pub server: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Type, TS)]
#[ts(export)]
#[sqlx(type_name = "ENUM")]
#[sqlx(rename_all = "snake_case")]
pub enum UserTransactionStatusEnum {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
    Refunded,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub enum PlanStatusEnum {
    Active,
    Disabled,
    Limited,
    Expired,
    OnHold,
}

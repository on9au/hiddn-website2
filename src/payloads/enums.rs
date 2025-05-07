use serde::Serialize;
use sqlx::prelude::Type;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Type, TS)]
#[ts(export)]
#[sqlx(type_name = "ENUM")]
#[sqlx(rename_all = "snake_case")]
pub enum UserTransactionStatus {
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

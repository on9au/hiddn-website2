use marzban_api::models::user::UserStatus;
use serde::Serialize;
use sqlx::prelude::Type;
use stripe::PaymentIntentStatus;
use ts_rs::TS;

#[derive(Clone, Debug, Copy, Serialize, Type, TS)]
#[ts(export)]
#[sqlx(type_name = "ENUM")]
#[sqlx(rename_all = "snake_case")]
pub enum UserTransactionStatus {
    Pending,
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
    Refunded,
}

impl From<PaymentIntentStatus> for UserTransactionStatus {
    fn from(status: PaymentIntentStatus) -> Self {
        match status {
            PaymentIntentStatus::Canceled => Self::Canceled,
            PaymentIntentStatus::Processing => Self::Processing,
            PaymentIntentStatus::RequiresAction => Self::RequiresAction,
            PaymentIntentStatus::RequiresCapture => Self::RequiresCapture,
            PaymentIntentStatus::RequiresConfirmation => Self::RequiresConfirmation,
            PaymentIntentStatus::RequiresPaymentMethod => Self::RequiresPaymentMethod,
            PaymentIntentStatus::Succeeded => Self::Succeeded,
        }
    }
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

impl From<UserStatus> for PlanStatusEnum {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Active => Self::Active,
            UserStatus::Disabled => Self::Disabled,
            UserStatus::Limited => Self::Limited,
            UserStatus::Expired => Self::Expired,
            UserStatus::OnHold => Self::OnHold,
        }
    }
}

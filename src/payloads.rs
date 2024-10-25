use serde::{Deserialize, Serialize};
use typeshare::U53;

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct LoginResponsePayload {
    pub logged_in: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct RegisterPayload {
    pub email: String,
    pub email_verification_code: String,
    pub password: String,
    pub confirm_password: String,
    pub invite_code: String,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct ForgotPasswordPayload {
    pub email: String,
    pub email_verification_code: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct VerifyEmailPayload {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub enum ServerStatusEnum {
    Online,
    Degraded,
    Unreachable,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct ServerStatusPayload {
    pub server: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct UserProfilePayload {
    pub email: String,
    pub email_verified: bool,
    pub created_at: String,
    pub updated_at: String,
    pub email_expiration_reminder: bool,
    pub email_data_reminder: bool,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct UserTransactionPayload {
    pub transaction_id: U53,
    pub amount: f64,
    pub transaction_date: String,
    pub payment_method: Option<String>,
    pub status: UserTransactionStatusEnum,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub plan_id: Option<U53>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub enum UserTransactionStatusEnum {
    Unpaid,
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct PlanDetailsPayload {
    pub expiration: String,
    pub status: PlanStatusEnum,
    #[serde(rename = "dataUsed")]
    pub data_used: f64,
    #[serde(rename = "dataLimit")]
    pub data_limit: f64,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub enum PlanStatusEnum {
    Active,
    Expired,
    Cancelled,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct AnnouncementPayload {
    pub id: U53,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct PlanPayload {
    pub id: U53,
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: U53,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct CreateOrderResponsePayload {
    pub order_id: U53,
    pub payment_intent_client_secret: String,
}

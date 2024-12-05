// payloads.rs

use chrono::{DateTime, Utc};
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
pub struct RequestCodePayload {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct VerifyEmailPayload {
    pub email: String,
    pub email_verification_code: String,
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

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct NewPlanPayload {
    pub name: String,
    pub price: f64,
    pub data_limit: Option<f64>,
    pub duration_days: U53,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct CreateOrderPayload {
    pub plan_id: U53,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct CreateOrderResponsePayload {
    pub order_id: u32,
    pub payment_intent_client_secret: String,
}

#[derive(Debug)]
pub struct SubscriptionPlan {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub data_limit: Option<i64>, // in GB
    pub duration_days: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct OnlineUser {
    pub id: u32,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub credit: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct UserSubscription {
    pub id: u32,
    pub online_user_id: i32,
    pub subscription_plan_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String, // 'active', 'expired', 'cancelled'
}

#[derive(Debug)]
pub struct PaymentTransaction {
    pub id: u32,
    pub online_user_id: u32,
    pub amount: f64,
    pub transaction_date: DateTime<Utc>,
    pub payment_method: Option<String>,
    pub status: String, // 'unpaid', 'pending', 'completed', 'failed', 'cancelled'
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub plan_id: Option<u32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[typeshare::typeshare]
pub struct AdminUser {
    pub id: u32,
    pub email: String,
    pub admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct AdminCreateAnnouncement {
    pub title: String,
    pub content: String,
}

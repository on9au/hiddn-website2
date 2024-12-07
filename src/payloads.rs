// payloads.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    mysql::{MySqlTypeInfo, MySqlValueRef},
    prelude::Type,
    Decode, Encode, MySql,
};
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

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct PasswordFeedbackPayload {
    pub warning: Option<String>,
    pub suggestions: Vec<String>,
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
    pub created_at: String,
    pub updated_at: String,
    pub email_expiration_reminder: bool,
    pub email_data_reminder: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct UserProfileSettingsChangePayload {
    pub email_expiration_reminder: Option<bool>,
    pub email_data_reminder: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub struct UserTransactionPayload {
    pub id: U53,
    pub user_id: U53,
    pub plan_id: U53,
    pub amount: f64,
    pub status: UserTransactionStatusEnum,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: U53, // Unix timestamp
    pub updated_at: U53,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserTransactionRust {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub amount: f64,
    pub status: UserTransactionStatusEnum,
    pub stripe_payment_intent_id: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Serialize, Type)]
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
    pub expiration: Option<U53>,
    pub status: PlanStatusEnum,
    #[serde(rename = "dataUsed")]
    pub data_used: U53,
    #[serde(rename = "dataLimit")]
    pub data_limit: Option<U53>,
}

/// PlanDetaulsPayload but with u64 instead of U53/u32 (since data used and data limit are in bytes
/// and will go over the limit of u32, and converting to U53 goes like: u64 -> u32 -> U53)
#[derive(Clone, Debug, Serialize)]
pub struct PlanDetailsRust {
    pub expiration: Option<u64>,
    pub status: PlanStatusEnum,
    #[serde(rename = "dataUsed")]
    pub data_used: u64,
    #[serde(rename = "dataLimit")]
    pub data_limit: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
#[typeshare::typeshare]
pub enum PlanStatusEnum {
    Active,
    Disabled,
    Limited,
    Expired,
    OnHold,
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
    pub marzban_username: Option<String>,
    pub admin: bool,
    pub created_at: U53,
    pub updated_at: U53,
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct AdminUserModify {
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserRust {
    pub id: u32,
    pub email: String,
    pub marzban_username: Option<String>,
    pub admin: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct AdminCreateAnnouncement {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[typeshare::typeshare]
pub struct ChangePasswordPayload {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

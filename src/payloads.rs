use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[typeshare::typeshare]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
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

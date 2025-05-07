use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct LoginResponsePayload {
    pub logged_in: bool,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct RegisterPayload {
    pub email: String,
    pub email_verification_code: String,
    pub password: String,
    pub confirm_password: String,
    pub invite_code: String,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct PasswordFeedbackPayload {
    pub warning: Option<String>,
    pub suggestions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct ForgotPasswordPayload {
    pub email: String,
    pub email_verification_code: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct RequestCodePayload {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct VerifyEmailPayload {
    pub email: String,
    pub email_verification_code: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ChangePasswordPayload {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

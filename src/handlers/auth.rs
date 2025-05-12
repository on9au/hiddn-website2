use crate::{
    config::GLOBAL_CONFIG,
    errors::AppResult,
    payloads::{
        ForgotPasswordPayload, LoginPayload, LoginResponse, PasswordFeedback, RegisterPayload,
        RequestCodePayload, VerifyEmailPayload,
    },
    security,
    sessions::AuthSession,
    state::AppState,
};
use anyhow::Context;
use axum::{Extension, Json, response::IntoResponse};
use chrono::Utc;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use reqwest::StatusCode;
use tracing::{debug, error};

/// POST `/api/auth/login`
pub async fn login(
    mut auth_session: AuthSession,
    Json(payload): Json<LoginPayload>,
) -> AppResult<impl IntoResponse> {
    // Authenticate the user with the db
    let user = match auth_session.authenticate(payload).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(StatusCode::UNAUTHORIZED.into_response()),
        Err(e) => {
            error!("Error: {}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    // Log the user in
    auth_session.login(&user).await?;

    Ok((StatusCode::OK, Json(LoginResponse { logged_in: true })).into_response())
}

/// POST `/api/auth/logout`
pub async fn logout(mut auth_session: AuthSession) -> AppResult<impl IntoResponse> {
    auth_session.logout().await?;
    Ok(StatusCode::OK.into_response())
}

/// GET `/api/auth/is-logged-in`
pub async fn is_logged_in(auth_session: AuthSession) -> AppResult<impl IntoResponse> {
    match auth_session.user {
        Some(_) => Ok(StatusCode::OK.into_response()),
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

/// POST `/api/auth/register`
pub async fn register_user(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    // Check if the user already exists
    if app_state
        .user_repository()
        .user_exists(&payload.email)
        .await?
    {
        return Ok(StatusCode::CONFLICT.into_response());
    }

    // Check if password is strong enough via zxcvbn
    let (score, warning, suggestions) =
        security::check_password_strength(&payload.password, &[&payload.email]);

    if score < zxcvbn::Score::Three {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning,
                suggestions,
            }),
        )
            .into_response());
    }

    // Check if both passwords match
    if payload.password != payload.confirm_password {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response());
    }

    // Check if email verification code is valid
    app_state
        .verification_repository()
        .is_verification_code_valid(&payload.email, &payload.email_verification_code)
        .await?;

    // All checks passed, create the user

    // Hash the password
    let password_hash = security::hash_password(&payload.password).await?;

    app_state
        .user_repository()
        .create_user(&payload.email, &password_hash, false)
        .await?;

    Ok(StatusCode::OK.into_response())
}

/// POST `/api/auth/forgot-password`
pub async fn forgot_password(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> AppResult<impl IntoResponse> {
    // Check if the user does not exist
    if !app_state
        .user_repository()
        .user_exists(&payload.email)
        .await?
    {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    // Check if password is strong enough via zxcvbn
    let (score, warning, suggestions) =
        security::check_password_strength(&payload.password, &[&payload.email]);

    if score < zxcvbn::Score::Three {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning,
                suggestions,
            }),
        )
            .into_response());
    }

    // Check if both passwords match
    if payload.password != payload.confirm_password {
        return Ok((
            StatusCode::CONFLICT,
            Json(PasswordFeedback {
                warning: "Passwords do not match".to_string().into(),
                suggestions: vec![],
            }),
        )
            .into_response());
    }

    // Check if email verification code is valid
    app_state
        .verification_repository()
        .is_verification_code_valid(&payload.email, &payload.email_verification_code)
        .await?;

    // All checks passed, change the user's password

    // Hash the password
    let password_hash = security::hash_password(&payload.password).await?;
    app_state
        .user_repository()
        .update_password_by_email(&payload.email, &password_hash)
        .await?;

    Ok(StatusCode::OK.into_response())
}

/// POST `/api/auth/email/request-code`
pub async fn request_code(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<RequestCodePayload>,
) -> AppResult<impl IntoResponse> {
    // Ensure that at least 30 seconds have passed since the last request
    let last_request = app_state
        .verification_repository()
        .get_last_verification_code_timestamp(&payload.email)
        .await?;

    // Ensure that at least 30 seconds have passed since the last request
    if let Some(timestamp) = last_request {
        let now = Utc::now();
        let duration = now - timestamp;
        if duration.num_seconds() < 30 {
            return Ok(StatusCode::TOO_MANY_REQUESTS.into_response());
        }
    }

    // Create the verification code
    let email_verification_code = app_state
        .verification_repository()
        .create_random_verification_code(&payload.email)
        .await?;

    // Create tera context
    let mut context = tera::Context::new();
    context.insert("code", &email_verification_code);

    // Render the email template
    let email_body = app_state
        .tera()
        .render("verification_email.html", &context)
        .context("Failed to render email template")?;

    // Create the email
    let email = Message::builder()
        .from(GLOBAL_CONFIG.from_email.parse().unwrap())
        .to(payload.email.parse().unwrap())
        .subject(GLOBAL_CONFIG.default_subject.clone())
        .header(ContentType::TEXT_HTML)
        .body(email_body)
        .context("Failed to create email")?;

    let creds = Credentials::new(
        GLOBAL_CONFIG.smtp_username.clone(),
        GLOBAL_CONFIG.smtp_password.clone(),
    );

    debug!("creating mailer");

    // Mail
    let mailer = SmtpTransport::relay(&GLOBAL_CONFIG.smtp_server)
        .context("Failed to create SMTP transport")?
        .credentials(creds)
        .build();

    debug!("Sending email to {}", payload.email);

    // Send the email.
    mailer.send(&email).context("Failed to send email")?;

    // Success, insert the code into the db
    app_state
        .verification_repository()
        .insert_verification_code(&payload.email, &email_verification_code)
        .await?;

    Ok(StatusCode::OK.into_response())
}

/// POST `/api/auth/email/verify`
pub async fn verify_email(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> AppResult<impl IntoResponse> {
    // Just check if the code is valid
    let is_valid = app_state
        .verification_repository()
        .is_verification_code_valid(&payload.email, &payload.email_verification_code)
        .await?;

    if !is_valid {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(StatusCode::OK.into_response())
}

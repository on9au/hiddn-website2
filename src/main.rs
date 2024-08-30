use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use payloads::{ForgotPasswordPayload, LoginPayload, RegisterPayload, VerifyEmailPayload};
use sessions::{AuthSession, Backend};
use tokio::net::TcpListener;

mod payloads;
mod sessions;

#[tokio::main]
async fn main() {
    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::default();
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .route("/", get(root))
        .route("/login_user", post(login_user))
        .route("/logout_user", post(logout_user))
        .route("/is_logged_in", get(is_logged_in))
        .layer(auth_layer)
        .route("/register_user", post(register_user))
        .route("/forgot_password", post(forgot_password))
        .route("/verify_email", post(verify_email));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Handler for the GET `/` route.
async fn root() -> &'static str {
    "Hello, World!"
}

/// Handler for the GET '/is_logged_in' route.
/// This handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// The server will use axum_login to keep the user authenticated.
async fn is_logged_in(auth_session: AuthSession) -> impl IntoResponse {
    println!("{:?}", auth_session.user);
    match auth_session.user {
        Some(_) => StatusCode::OK.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

/// Handler for the POST '/login_user' route.
/// This handler will receive a JSON(LoginPayload) payload from the client.
/// The handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// If email is invalid, it will return BAD_REQUEST.
/// The server will use axum_login to keep the user authenticated.
async fn login_user(
    mut auth_session: AuthSession,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // TODO: Implement actual authentication logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    let user = match auth_session.authenticate(payload).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(e) => {
            println!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match auth_session.login(&user).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    StatusCode::OK.into_response()
}

/// Handler for the POST '/logout_user' route.
/// This handler will log the user out, clearing the session.
/// axum_login will handle the session management, logging the user out.
async fn logout_user(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            println!("Error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler for the POST '/verify_email' route.
/// This handler will receive a JSON(VerifyEmailPayload) payload from the client.
/// It will send code to email to verify the email.
/// Should have a rate limit to prevent spamming.
/// This acts as a way to verify the email's ownership and existence.
async fn verify_email(Json(payload): Json<VerifyEmailPayload>) -> impl IntoResponse {
    // TODO: Implement actual email verification logic
    // We would create a temporary verificaton code linked to the email in the db which expires after a certain time.
    // We would send the verification code to the email.

    // Debug print the payload
    println!("{:?}", payload);

    StatusCode::OK.into_response()
}

/// Handler for the POST '/register_user' route.
/// This handler will receive a JSON(RegisterPayload) payload from the client.
/// The handler will return OK if the user is registered.
/// If email is already taken, it will return BAD_REQUEST.
/// If email is invalid, it will return BAD_REQUEST.
/// If password is invalid, it will return CONFLICT.
/// If password is too weak, it will return CONFLICT.
/// If verification code is invalid, it will return FORBIDDEN.
/// The server will use axum_login to keep the user authenticated.
async fn register_user(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    // TODO: Implement actual registration logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    // Check if the email is already taken
    println!("Checking if email is already taken...");

    // Check if code is valid
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Check if password is valid
    // Check if password is too weak
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    // We can use regex to check this.
    // We can also use a library like zxcvbn to check password strength.
    // For now, we will just check if the password is at least 8 characters long.
    if payload.password.len() < 8 {
        return StatusCode::CONFLICT.into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return StatusCode::CONFLICT.into_response();
    }

    // Register the user
    // We would hash the password before storing it in the db.
    // Verification code can be safely discarded after registration.
    // New db entry for the user. After, use axum_login to authenticate the user.
    // db new user
    // let registered_user = that db entry into User struct

    // Return OK, user is registered, client must now login.
    StatusCode::OK.into_response()
}

/// Handler for the POST '/forgot_password' route.
/// This handler will receive a JSON(ForgotPassword) payload from the client.
/// The handler will return OK if the user password is reset.
/// If user does not exist, it will return NOT_FOUND.
/// If password is invalid, it will return CONFLICT.
/// If password is too weak, it will return CONFLICT.
/// If verification code is invalid, it will return FORBIDDEN.
/// The server will use axum_login to keep the user authenticated.
async fn forgot_password(Json(payload): Json<ForgotPasswordPayload>) -> impl IntoResponse {
    // TODO: Implement actual registration logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    // Check if the user exists
    if payload.email != "test@test.com" {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Check if code is valid
    if payload.email_verification_code != "123456" {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Check if password is valid
    // Check if password is too weak
    // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
    // We can use regex to check this.
    // We can also use a library like zxcvbn to check password strength.
    // For now, we will just check if the password is at least 8 characters long.
    if payload.password.len() < 8 {
        return StatusCode::CONFLICT.into_response();
    }

    // Check if password and confirm password match
    if payload.password != payload.confirm_password {
        return StatusCode::CONFLICT.into_response();
    }

    // Register the user
    // We would hash the password before storing it in the db.
    // Verification code can be safely discarded after registration.
    // New db entry for the user. After, use axum_login to authenticate the user.
    // db new user
    // let registered_user = that db entry into User struct

    // Return OK, user is registered, client must now login.
    StatusCode::OK.into_response()
}

use axum::{
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use payloads::{LoginPayload, RegisterPayload, VerifyEmailPayload};
use tokio::net::TcpListener;

mod payloads;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/login_user", post(login_user))
        .route("/register_user", post(register_user))
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

/// Handler for the POST '/login_user' route.
/// This handler will receive a JSON(LoginPayload) payload from the client.
/// The handler will return OK if the user is authenticated, and UNAUTHORIZED if the user is not.
/// If email is invalid, it will return BAD_REQUEST.
/// The server will send cookies to the client to keep the user authenticated.
async fn login_user(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    // TODO: Implement actual authentication logic interfacing with db

    // Debug print the payload
    println!("{:?}", payload);

    // Check if email is valid
    if !payload.email.contains('@') {
        return StatusCode::BAD_REQUEST.into_response();
    }

    // Check if email is test@test.com, if so, return OK and set cookies
    // In a real application, this would be replaced with actual authentication logic
    // Would interface with a database to check if the email and password are valid
    if payload.email == *"test@test.com" {
        // Set cookies on successful authentication
        // In a real application, a session token would be generated and stored in a database
        let header_value: HeaderValue =
            // USE HTTP ONLY AND SECURE FLAGS IN PRODUCTION
            match HeaderValue::from_str(format!("auth_token={}", "test_auth_token").as_str()) {
                Ok(value) => value,
                Err(_) => {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

        return (AppendHeaders([(SET_COOKIE, header_value)]), StatusCode::OK).into_response();
    }
    StatusCode::UNAUTHORIZED.into_response()
}

/// Handler for the POST '/register_user' route.
/// This handler will receive a JSON(RegisterPayload) payload from the client.
/// The handler will return OK if the user is registered, and BAD_REQUEST if the user is not.
/// The server will send cookies to the client to keep the user authenticated.
async fn register_user(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    todo!("Implement register_user handler");
}

/// Handler for the POST '/verify_email' route.
/// This handler will receive a JSON(VerifyEmailPayload) payload from the client.
/// It will send code to email to verify the email.
/// Should have a rate limit to prevent spamming.
async fn verify_email(Json(payload): Json<VerifyEmailPayload>) -> impl IntoResponse {
    todo!("Implement verify_email handler");
}

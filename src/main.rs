use axum::{
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use payloads::LoginPayload;
use tokio::net::TcpListener;

mod payloads;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/login_user", post(login_user));

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
/// This handler will receive a JSON object with the following structure:
/// {
///    "email": string,
///    "password": string
/// }
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
        let mut headers = HeaderMap::new();
        let header_value: HeaderValue = match HeaderValue::from_str(
            format!("auth_token={}; HttpOnly; Secure", "test_auth_token").as_str(),
        ) {
            Ok(value) => value,
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        headers.insert(SET_COOKIE, header_value);

        return StatusCode::OK.into_response();
    }
    StatusCode::UNAUTHORIZED.into_response()
}

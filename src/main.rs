use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

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
/// The server will send cookies to the client to keep the user authenticated.
async fn login_user() -> &'static str {
    "Hello, World!"
}

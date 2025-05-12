//! # Anyhow Error Wrapper for Axum

use anyhow::Error;
use axum::response::{IntoResponse, Response};
use tracing::{debug, error};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub struct AppError(pub Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Customize this as needed:
        error!("Internal error: {:?}", self.0);
        for cause in self.0.chain() {
            debug!("Error cause: {}", cause);
        }
        debug!("Error backtrace: {}", self.0.backtrace());
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}

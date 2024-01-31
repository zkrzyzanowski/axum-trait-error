use axum::debug_handler;

use axum::{http::StatusCode, response::IntoResponse, response::Response};
use std::fmt;

use error::error::CustomError;

#[debug_handler]
pub async fn run(_body: String) -> Result<String, CustomError> {
    Ok("".to_string())
}

// #[derive(Debug)]
// pub struct CustomError(anyhow::Error);

// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "")
//     }
// }

// impl std::error::Error for CustomError {}

// impl IntoResponse for CustomError {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Something went wrong: {}", self.0),
//         )
//             .into_response()
//     }
// }

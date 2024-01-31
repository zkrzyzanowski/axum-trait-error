use axum::debug_handler;

use error::error::CustomError;

#[debug_handler]
pub async fn run(_body: String) -> Result<String, CustomError> {
    Ok("".to_string())
}

use axum::{
    debug_handler, error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse,
    response::Response, routing::post, BoxError, Router,
};
use std::fmt;
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::catch_panic::CatchPanicLayer;

// comment this out after uncommenting CustomError code below
use error::error::CustomError;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(CatchPanicLayer::new())
        .route("/test", post(run));

    let addr = format!("0.0.0.0:8080",);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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

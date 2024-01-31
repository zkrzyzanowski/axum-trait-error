use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::post, BoxError, Router};
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::catch_panic::CatchPanicLayer;

mod routes;
use routes::route;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(CatchPanicLayer::new())
        .route("/test", post(route::run))
        .layer(
            ServiceBuilder::new()
                // this middleware goes above `TimeoutLayer` because it will receive
                // errors returned by `TimeoutLayer`
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(20))),
        );

    let addr = format!("0.0.0.0:8080",);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

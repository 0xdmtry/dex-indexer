use crate::routes::v1;
use crate::state::AppState;
use axum::http::Method;
use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub async fn app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .nest("/v1", v1::routes())
        .layer(cors)
        .with_state(state)
}

pub async fn server(state: AppState) {
    let app = app(state)
        .await
        .into_make_service_with_connect_info::<SocketAddr>();
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Cannot bind port 8000");
    serve(listener, app).await.expect("Cannot serve");
}

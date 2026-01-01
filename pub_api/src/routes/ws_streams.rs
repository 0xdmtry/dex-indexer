use crate::handlers::ws_creation_stream_handler::ws_creation;
use crate::handlers::ws_migration_stream_handler::ws_migration;
use crate::handlers::ws_ping_stream_handler::ws_ping;
use crate::handlers::ws_price_stream_handler::ws_price;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ws/ping", get(ws_ping))
        .route("/ws/price/{identifier}", get(ws_price))
        .route("/ws/creation", get(ws_creation))
        .route("/ws/migration", get(ws_migration))
}

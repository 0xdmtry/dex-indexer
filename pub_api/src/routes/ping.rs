use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/ping", get(|| async { "pub_api::v0.0.2" }))
}

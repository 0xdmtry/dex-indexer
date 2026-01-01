use crate::handlers::prices_handler::get_price;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/price/{bonding_curve}", get(get_price))
}

use crate::handlers::bonding_curves_handler::get_bonding_curve;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/bonding-curve/{mint}", get(get_bonding_curve))
}

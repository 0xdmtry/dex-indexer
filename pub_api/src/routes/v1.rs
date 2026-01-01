use super::{bonding_curves, ping, prices, ws_streams};
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes())
        .merge(prices::routes())
        .merge(bonding_curves::routes())
        .merge(ws_streams::routes())
}

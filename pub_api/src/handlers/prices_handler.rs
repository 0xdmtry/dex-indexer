use crate::state::AppState;
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use redis::AsyncCommands;
use serde_json::json;

pub async fn get_price(
    State(state): State<AppState>,
    Path(bonding_curve): Path<String>,
) -> impl IntoResponse {
    let mut conn = state.cache.clone();

    let key = format!("prices:{bonding_curve}");

    match conn.get::<_, String>(key).await {
        Ok(value) => (
            StatusCode::OK,
            Json(json!({ "bonding_curve": bonding_curve, "price": value })),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "bonding_curve": bonding_curve, "error": "not found" })),
        ),
    }
}

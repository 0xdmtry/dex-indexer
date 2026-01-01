use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use redis::AsyncCommands;
use serde_json::json;

pub async fn get_bonding_curve(
    State(state): State<AppState>,
    Path(mint): Path<String>,
) -> impl IntoResponse {
    let mut conn = state.cache.clone();

    let key = format!("bonding:{mint}");

    match conn.get::<_, String>(key).await {
        Ok(value) => (
            StatusCode::OK,
            Json(json!({ "mint": mint, "bonding": value })),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "mint": mint, "error": "not found" })),
        ),
    }
}

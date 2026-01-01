use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::response::IntoResponse;
use std::time::Duration;
use tokio::time;

pub async fn ws_ping(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_ws_ping)
}

async fn handle_ws_ping(mut socket: WebSocket) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if socket
            .send(Message::Text(Utf8Bytes::from("pub_api::v0.0.3")))
            .await
            .is_err()
        {
            break;
        }
    }
}

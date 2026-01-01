use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio_stream::StreamExt;

async fn handle_ws_creation(mut socket: WebSocket, redis_url: String) {
    let client = redis::Client::open(redis_url).unwrap();
    let mut pubsub = client.get_async_pubsub().await.unwrap();
    pubsub.subscribe("creation").await.unwrap();

    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        if let Ok(payload) = msg.get_payload::<String>() {
            if socket.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    }
}

pub async fn ws_creation(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    let redis_url = state.config.redis_url.clone();
    ws.on_upgrade(move |socket| async move {
        handle_ws_creation(socket, redis_url).await;
    })
}

use crate::models::resolver::EnrichedResolvedToken;
use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use log::error;
use redis::AsyncCommands;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub async fn ws_price(
    State(state): State<AppState>,
    Path(identifier): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let redis_url = state.config.redis_url.clone();

    ws.on_upgrade(move |socket| async move {
        let channel_id = Uuid::new_v4().to_string();

        publish_identifier(&state, identifier.clone(), channel_id.clone())
            .await
            .unwrap();

        subscribe_for_price(socket, redis_url, channel_id.clone()).await;
    })
}

async fn publish_identifier(
    state: &AppState,
    identifier: String,
    channel_id: String,
) -> redis::RedisResult<()> {
    let mut cache = state.cache.clone();
    let list: Vec<String> = vec![identifier.clone(), channel_id.clone()];

    let payload = serde_json::to_string(&list).unwrap(); // TODO re-do unwrap()

    let _: () = cache.publish("req_handler", payload).await?;

    Ok(())
}

async fn subscribe_for_price(mut socket: WebSocket, redis_url: String, channel_id: String) {
    let redis_url_clone = redis_url.clone();
    let client = redis::Client::open(redis_url).unwrap();
    let mut pubsub = client.get_async_pubsub().await.unwrap();

    pubsub.subscribe(&channel_id).await.unwrap();

    {
        let mut stream = pubsub.on_message(); // borrows pubsub mutably

        if let Some(msg) = stream.next().await {
            let token: EnrichedResolvedToken =
                serde_json::from_str(msg.get_payload::<String>().unwrap().as_str()).unwrap();
            send_init_price(&mut socket, token.clone()).await;
            create_price_subscription(socket, redis_url_clone, token.clone()).await;
        }
    }

    pubsub.unsubscribe(&channel_id).await.unwrap();
}

async fn create_price_subscription(
    mut socket: WebSocket,
    redis_url: String,
    token: EnrichedResolvedToken,
) {
    // TODO review the redis connection — make it robust
    let client = redis::Client::open(redis_url).unwrap();
    let mut pubsub = client.get_async_pubsub().await.unwrap();
    let mint: String = token.mint;

    let channel = format!("ws:{mint}");
    pubsub.subscribe(&channel).await.unwrap();

    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        if let Ok(payload) = msg.get_payload::<String>() {
            if socket.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    }
}

async fn send_init_price(socket: &mut WebSocket, token: EnrichedResolvedToken) {
    let token_str = serde_json::to_string(&token).unwrap(); // TODO re-do the unwrap()

    if socket.send(Message::Text(token_str.into())).await.is_err() {
        error!("send_init_price::socket::send: SendError");
    }
}

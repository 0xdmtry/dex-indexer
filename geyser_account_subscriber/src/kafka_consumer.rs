use crate::config::AppConfig;
use crate::models::kafka_req::KReq;
use crate::state::AppState;
use log::{error, info, warn};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::time::Duration;
use tokio::time::sleep;

const TOPICS: &[&str] = &["price_req"];
const MAX_RETRIES: u32 = 30;
const RETRY_DELAY_SECS: u64 = 2;

pub async fn start_kafka_consumer(config: AppConfig, state: AppState) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &config.kafka_group_id)
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create Kafka consumer");

    // Retry subscription until topic is available
    for attempt in 1..=MAX_RETRIES {
        match consumer.subscribe(TOPICS) {
            Ok(_) => {
                info!("Kafka consumer subscribed to topics: {TOPICS:?}");
                break;
            }
            Err(e) => {
                warn!("Subscribe attempt {attempt}/{MAX_RETRIES} failed: {e}. Retrying...");
                if attempt == MAX_RETRIES {
                    panic!("Failed to subscribe after {MAX_RETRIES} attempts");
                }
                sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
            }
        }
    }

    loop {
        match consumer.recv().await {
            Ok(message) => {
                let topic = message.topic();
                let payload = match message.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        error!("Error deserializing message payload: {e:?}");
                        continue;
                    }
                    None => {
                        warn!("Empty message payload");
                        continue;
                    }
                };

                if let Err(e) = handle_message(topic, payload, &state).await {
                    error!("Failed to handle message from {topic}: {e}");
                }
            }
            Err(e) => {
                error!("Kafka consumer error: {e:?}");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn handle_message(topic: &str, payload: &str, state: &AppState) -> anyhow::Result<()> {
    match topic {
        "price_req" => {
            let kreq: KReq = serde_json::from_str(payload)?;
            let handle = state.subscription_handle.clone();
            tokio::spawn(async move {
                match handle.update_subscription(kreq.tracked_accounts).await {
                    Ok(_) => {
                        info!("SUCCESS");
                    }
                    Err(e) => {
                        error!("Failed to update subscription: {e}");
                    }
                }
            });
        }
        _ => {
            warn!("Unknown topic: {topic}")
        }
    }
    Ok(())
}

use crate::config::AppConfig;
use crate::models::kafka_event::{KEvent, KEventType};
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

const PF_PRICE_UPDATED: &[&str] = &["pf_price_update_event"];
const PS_PRICE_UPDATED: &[&str] = &["ps_price_update_event"];
const RLL_PRICE_UPDATED: &[&str] = &["rll_price_update_event"];

pub async fn start_kafka_producer(
    config: AppConfig,
    mut event_rx: tokio::sync::mpsc::Receiver<KEvent>,
) {
    info!("Starting Kafka producer");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");

    tokio::spawn(async move {
        while let Some(kevent) = event_rx.recv().await {
            match kevent.event_type {
                KEventType::PfPriceUpdated => {
                    broadcast_event(&producer, PF_PRICE_UPDATED, &kevent).await;
                }
                KEventType::PsPriceUpdated => {
                    broadcast_event(&producer, PS_PRICE_UPDATED, &kevent).await;
                }
                KEventType::RllPriceUpdated => {
                    broadcast_event(&producer, RLL_PRICE_UPDATED, &kevent).await;
                }
            }
        }
    });
}

async fn broadcast_event(producer: &FutureProducer, topics: &[&str], event: &KEvent) {
    for topic in topics {
        match send_event(producer, topic, event).await {
            Ok(_) => {
                info!("Successfully sent an event to {topic}")
            }
            Err(e) => {
                error!("Failed to send an event to {topic}: {e}")
            }
        }
    }
}

async fn send_event(producer: &FutureProducer, topic: &str, event: &KEvent) -> Result<(), String> {
    info!("topic: {topic}; event: {event:?};");

    let payload =
        serde_json::to_string(event).map_err(|e| format!("Failed to serialize event: {e}"))?;

    let record = FutureRecord::to(topic).payload(&payload).key("test_key");

    producer
        .send(record, Duration::from_secs(0))
        .await
        .map_err(|(e, _)| format!("Failed to send to Kafka: {e}"))?;

    Ok(())
}

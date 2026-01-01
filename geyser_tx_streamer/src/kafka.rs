use crate::config::AppConfig;
use crate::models::consts::{
    PUMPFUN_CREATE_EVENT_TOPICS, PUMPFUN_MIGRATE_EVENT_TOPICS, PUMPFUN_TRADE_EVENT_TOPICS,
    PUMPSWAP_TRADE_EVENT_TOPICS, RAYDIUM_LAUNCHLAB_CREATE_EVENT_TOPICS,
    RAYDIUM_LAUNCHLAB_MIGRATE_EVENT_TOPICSS, RAYDIUM_LAUNCHLAB_TRADE_EVENT_TOPICSS,
};
use crate::models::kafka_event::{KEvent, KEventType};
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn start_kafka_producer(
    config: AppConfig,
    mut event_rx: tokio::sync::mpsc::Receiver<KEvent>,
) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");

    tokio::spawn(async move {
        while let Some(kevent) = event_rx.recv().await {
            match kevent.event_type {
                KEventType::PfChTradeOccurred => {
                    broadcast_event(&producer, PUMPFUN_TRADE_EVENT_TOPICS, &kevent).await;
                }
                KEventType::PfTradeOccurred => {
                    broadcast_event(&producer, PUMPFUN_TRADE_EVENT_TOPICS, &kevent).await;
                }
                KEventType::PfTokenCreated => {
                    broadcast_event(&producer, PUMPFUN_CREATE_EVENT_TOPICS, &kevent).await;
                }
                KEventType::PfTokenMigrated => {
                    broadcast_event(&producer, PUMPFUN_MIGRATE_EVENT_TOPICS, &kevent).await;
                }
                KEventType::PsTradeOccurred => {
                    broadcast_event(&producer, PUMPSWAP_TRADE_EVENT_TOPICS, &kevent).await;
                }
                KEventType::RllTradeOccurred => {
                    broadcast_event(&producer, RAYDIUM_LAUNCHLAB_TRADE_EVENT_TOPICSS, &kevent)
                        .await;
                }
                KEventType::RllTokenCreated => {
                    broadcast_event(&producer, RAYDIUM_LAUNCHLAB_CREATE_EVENT_TOPICS, &kevent)
                        .await;
                }
                KEventType::RllTokenMigrated => {
                    broadcast_event(&producer, RAYDIUM_LAUNCHLAB_MIGRATE_EVENT_TOPICSS, &kevent)
                        .await;
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
    let payload =
        serde_json::to_string(event).map_err(|e| format!("Failed to serialize event: {e}"))?;

    let record = FutureRecord::to(topic).payload(&payload).key("test_key");

    producer
        .send(record, Duration::from_secs(0))
        .await
        .map_err(|(e, _)| format!("PUMP::TX::send_event::Failed to send to Kafka: {e}"))?;

    Ok(())
}

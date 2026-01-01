use crate::config::AppConfig;
use crate::models::enums::KReqType;
use crate::models::kafka::KReq;
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn start_kafka_producer(
    config: AppConfig,
    mut req_rx: tokio::sync::mpsc::Receiver<KReq>,
) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");

    tokio::spawn(async move {
        while let Some(kreq) = req_rx.recv().await {
            match kreq.req_type {
                KReqType::PriceReqBondingCurve => {
                    match send_price_req(&producer, "pumpfun_price_req", &kreq).await {
                        Ok(_) => {
                            info!("Successfully sent a req to `pumpfun_price_req`")
                        }
                        Err(e) => {
                            error!("Failed to send a req to `pumpfun_price_req`: {e}")
                        }
                    }
                }
                KReqType::PriceReqPool => {
                    match send_price_req(&producer, "pumpswap_price_req", &kreq).await {
                        Ok(_) => {
                            info!("Successfully sent a req to `pumpswap_price_req`")
                        }
                        Err(e) => {
                            error!("Failed to send a req to `pumpswap_price_req`: {e}")
                        }
                    }
                }
                KReqType::PriceReqPoolState => {
                    match send_price_req(&producer, "raydium_launchlab_price_req", &kreq).await {
                        Ok(_) => {
                            info!("Successfully sent a req to `raydium_launchlab_price_req`")
                        }
                        Err(e) => {
                            error!("Failed to send a req to `raydium_launchlab_price_req`: {e}")
                        }
                    }
                }
                KReqType::TokenReqFulfill => {
                    match send_price_req(&producer, "fulfill_req", &kreq).await {
                        Ok(_) => {
                            info!("Successfully sent a req to `fulfill_req`")
                        }
                        Err(e) => {
                            error!("Failed to send a req to `fulfill_req`: {e}")
                        }
                    }
                }
            }
        }
    });
}

async fn send_price_req(producer: &FutureProducer, topic: &str, kreq: &KReq) -> Result<(), String> {
    let payload =
        serde_json::to_string(kreq).map_err(|e| format!("Failed to serialize req: {e}"))?;

    let record = FutureRecord::to(topic).payload(&payload).key("test_key");

    producer
        .send(record, Duration::from_secs(0))
        .await
        .map_err(|(e, _)| format!("PUMP::REQ::send_req::Failed to send to Kafka: {e}"))?;

    Ok(())
}

use geyser_account_subscriber::api::geyser::subscription_manager::SubscriptionManager;
use geyser_account_subscriber::cache::{get_subscriptions, init_cache};
use geyser_account_subscriber::config::AppConfig;
use geyser_account_subscriber::kafka_consumer::start_kafka_consumer;
use geyser_account_subscriber::kafka_producer::start_kafka_producer;
use geyser_account_subscriber::models::enums::Platform;
use geyser_account_subscriber::models::kafka_event::KEvent;
use geyser_account_subscriber::state::init_state;
use log::{error, info};
use std::collections::HashMap;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = AppConfig::from_env();
    let config_clone = config.clone();

    let cache = init_cache(config.clone()).await.unwrap();
    let _tracked_accounts = get_subscriptions(&mut cache.clone()).await.unwrap();

    let (event_tx, event_rx) = mpsc::channel::<KEvent>(1000); // TODO review the 1000 buffer size
    let (_manager_task, subscription_handle) = SubscriptionManager::spawn(
        config_clone.geyser_url.clone(),
        config_clone.geyser_token.clone(),
        event_tx,
    );

    let state = init_state(config_clone.clone(), subscription_handle.clone(), cache);

    let tracked_accounts = HashMap::from([(
        "8sbZehpLFcjCGk7vQGmBVbVVyih4pQLF6NuYnp61jXMF".to_string(),
        Platform::PumpFun,
    )]);

    if !tracked_accounts.is_empty() {
        tokio::spawn(async move {
            match subscription_handle
                .update_subscription(tracked_accounts)
                .await
            {
                Ok(_) => {
                    info!("SUCCESS");
                }
                Err(e) => {
                    error!("Failed to update subscription: {e}");
                }
            }
        });
    }

    tokio::spawn(async move {
        start_kafka_producer(config_clone.clone(), event_rx).await;
    });

    start_kafka_consumer(config, state).await;
}

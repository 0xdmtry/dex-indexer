use geyser_tx_streamer::api::geyser::tx_consumer::TxConsumer;
use geyser_tx_streamer::config::AppConfig;
use geyser_tx_streamer::kafka::start_kafka_producer;
use geyser_tx_streamer::models::kafka_event::KEvent;
use geyser_tx_streamer::state::init_state;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = AppConfig::from_env();
    let _state = init_state(config.clone());
    let config_clone = config.clone();
    let (event_tx, event_rx) = mpsc::channel::<KEvent>(10_000);

    tokio::spawn(async move {
        start_kafka_producer(config_clone, event_rx).await;
    });

    let geyser_url = config.geyser_url.clone();
    let geyser_token = config.geyser_token.clone();

    TxConsumer {
        geyser_url,
        geyser_token,
        event_tx,
    }
    .start()
    .await;
}

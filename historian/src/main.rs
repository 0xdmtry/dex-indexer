use historian::cache::init_cache;
use historian::config::AppConfig;
use historian::db::init_db;
use historian::handlers::cache_handler::handle_price_req;
use historian::kafka::start_kafka_producer;
use historian::models::kafka::KReq;
use historian::rpc::init_rpc;
use historian::state::init_state;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = AppConfig::from_env();
    let rpc = init_rpc(config.clone()).await;
    let cache = init_cache(config.clone()).await.unwrap();
    let pg_pool = init_db(config.clone()).await;

    let state = init_state(config.clone(), rpc, cache, pg_pool);
    let config_clone = config.clone();
    let (req_tx, req_rx) = mpsc::channel::<KReq>(10_000);

    tokio::spawn(async move {
        start_kafka_producer(config, req_rx).await;
    });

    handle_price_req(state.clone(), config_clone.redis_url.clone(), req_tx).await;
}

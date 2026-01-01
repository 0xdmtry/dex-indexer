use data_processor::cache::init_cache;
use data_processor::clickhouse::init_clickhouse_client;
use data_processor::config::AppConfig;
use data_processor::db::init_db;
use data_processor::kafka::start_kafka_consumer;
use data_processor::state::init_state;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::from_env();
    let cache = init_cache(config.clone()).await.unwrap();
    let pg_pool = init_db(config.clone()).await;

    let clickhouse = init_clickhouse_client(&config).unwrap();
    let state = init_state(config.clone(), pg_pool, clickhouse, cache);

    start_kafka_consumer(config, state).await;
}

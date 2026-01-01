use pub_api::cache::init_cache;
use pub_api::config::AppConfig;
use pub_api::{app, state::init_state};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::from_env();
    let cache = init_cache(config.clone()).await.unwrap();
    let state = init_state(config.clone(), cache);

    app::server(state).await;
}

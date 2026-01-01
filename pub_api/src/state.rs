use crate::config::AppConfig;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub cache: ConnectionManager,
}

pub fn init_state(config: AppConfig, cache: ConnectionManager) -> AppState {
    AppState { config, cache }
}

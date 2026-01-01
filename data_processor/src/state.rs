use crate::config::AppConfig;
use clickhouse::Client;
use redis::aio::ConnectionManager;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pg_pool: sqlx::PgPool,
    pub cache: ConnectionManager,
    pub clickhouse: Client,
}

pub fn init_state(
    config: AppConfig,
    pg_pool: PgPool,
    clickhouse: Client,
    cache: ConnectionManager,
) -> AppState {
    AppState {
        config,
        pg_pool,
        clickhouse,
        cache,
    }
}

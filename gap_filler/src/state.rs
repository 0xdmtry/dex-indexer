use crate::config::AppConfig;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub rpc_client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    pub cache: ConnectionManager,
    pub pg_pool: sqlx::PgPool,
}

pub fn init_state(
    config: AppConfig,
    rpc_client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    cache: ConnectionManager,
    pg_pool: PgPool,
) -> AppState {
    AppState {
        config,
        rpc_client,
        cache,
        pg_pool,
    }
}

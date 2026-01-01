use crate::config::AppConfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::sync::Arc;

pub async fn init_rpc(config: AppConfig) -> Arc<RpcClient> {
    Arc::new(RpcClient::new(config.rpc_http_url))
}

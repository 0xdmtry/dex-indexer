use crate::config::AppConfig;
use log::{error, info};
use redis::Client;
use redis::aio::ConnectionManager;

pub async fn init_cache(config: AppConfig) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(config.redis_url).map_err(|e| {
        error!("Failed to create Redis client: {e}");
        e
    })?;
    let conn = client.get_connection_manager().await.map_err(|e| {
        error!("Failed to connect to Redis: {e}");
        e
    })?;
    info!("Connected to Redis successfully.");
    Ok(conn)
}

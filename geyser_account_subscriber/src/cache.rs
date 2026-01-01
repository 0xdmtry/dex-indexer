use crate::config::AppConfig;
use crate::models::enums::Platform;
use log::{error, info};
use redis::AsyncCommands;
use redis::Client;
use redis::aio::ConnectionManager;
use std::collections::HashMap;

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

pub async fn get_subscriptions(
    conn: &mut ConnectionManager,
) -> redis::RedisResult<HashMap<String, Platform>> {
    let keys = vec![
        "subscriptions:by_bonding_curve",
        "subscriptions:by_pool",
        "subscriptions:by_pool_state",
    ];

    let mut map = HashMap::new();

    for key in keys {
        if let Ok(value) = conn.get::<_, String>(key).await {
            if let Ok(platform) = serde_json::from_str::<Platform>(&value) {
                map.insert(key.to_string(), platform);
            }
        }
    }

    Ok(map)
}

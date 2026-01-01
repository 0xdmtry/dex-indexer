use crate::models::cache::{CacheError, TokenData};
use crate::models::classifier::StringType;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;

pub async fn fetch_from_cache(
    cache: &mut ConnectionManager,
    input: &str,
    string_type: &StringType,
) -> Result<Vec<TokenData>, CacheError> {
    let key = match string_type {
        StringType::Name => format!("token:by_name:{}", input.trim().to_lowercase()),
        StringType::Symbol => format!("token:by_symbol:{}", input.trim().to_lowercase()),
        StringType::Address => {
            // Try all three address types
            if let Some(data) = cache
                .get::<_, Option<String>>(&format!("tokens:by_mint:{input}"))
                .await?
            {
                let token: TokenData = serde_json::from_str(&data)?;
                return Ok(vec![token]);
            }
            if let Some(data) = cache
                .get::<_, Option<String>>(&format!("tokens:by_bonding_curve:{input}"))
                .await?
            {
                let token: TokenData = serde_json::from_str(&data)?;
                return Ok(vec![token]);
            }
            if let Some(data) = cache
                .get::<_, Option<String>>(&format!("tokens:by_pool:{input}"))
                .await?
            {
                let token: TokenData = serde_json::from_str(&data)?;
                return Ok(vec![token]);
            }
            // TODO refactoring the cache keys — they aren't supposed to be hardcoded
            if let Some(data) = cache
                .get::<_, Option<String>>(&format!("tokens:by_pool_state:{input}"))
                .await?
            {
                let token: TokenData = serde_json::from_str(&data)?;
                return Ok(vec![token]);
            }
            return Ok(Vec::new());
        }
    };

    let data: Option<String> = cache.get(&key).await?;
    match data {
        Some(json) => {
            let tokens: Vec<TokenData> = serde_json::from_str(&json)?;
            Ok(tokens)
        }
        None => Ok(Vec::new()),
    }
}

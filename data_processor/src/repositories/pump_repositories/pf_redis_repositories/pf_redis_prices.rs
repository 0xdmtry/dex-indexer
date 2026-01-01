use crate::models::pump_models::pf_models::pf_redis::{PfRedisBondingCurveProgress, PfRedisPrice};
use crate::state::AppState;
use chrono::Utc;
use redis::AsyncCommands;

/// Upsert `PfRedisPrice` into Redis without clobbering metadata fields
pub async fn upsert_pf_redis_price(
    state: &AppState,
    incoming: PfRedisPrice,
) -> Result<(), redis::RedisError> {
    let mut redis = state.cache.clone();
    // Fetch existing value (if any)
    let existing_json: Option<String> = redis.hget("pf_prices", &incoming.mint).await?;

    let merged = if let Some(json) = existing_json {
        let mut existing: PfRedisPrice = serde_json::from_str(&json).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "failed to deserialize PfRedisPrice",
                e.to_string(),
            ))
        })?;

        // ===== overwrite trade-derived fields =====
        existing.bonding_curve = incoming.bonding_curve;
        existing.price = incoming.price;
        existing.source = incoming.source;
        existing.direction = incoming.direction;
        existing.decimals = incoming.decimals;
        existing.virtual_token_reserves = incoming.virtual_token_reserves;
        existing.virtual_sol_reserves = incoming.virtual_sol_reserves;
        existing.real_token_reserves = incoming.real_token_reserves;
        existing.real_sol_reserves = incoming.real_sol_reserves;
        existing.ts = incoming.ts;
        existing.updated_at = Utc::now();

        // ===== preserve metadata unless explicitly provided =====
        if incoming.name.is_some() {
            existing.name = incoming.name;
        }
        if incoming.symbol.is_some() {
            existing.symbol = incoming.symbol;
        }
        if incoming.uri.is_some() {
            existing.uri = incoming.uri;
        }
        if incoming.description.is_some() {
            existing.description = incoming.description;
        }
        if incoming.twitter.is_some() {
            existing.twitter = incoming.twitter;
        }
        if incoming.telegram.is_some() {
            existing.telegram = incoming.telegram;
        }
        if incoming.website.is_some() {
            existing.website = incoming.website;
        }
        if incoming.image.is_some() {
            existing.image = incoming.image;
        }

        existing
    } else {
        // First insert — safe to store as-is
        incoming
    };

    let value = serde_json::to_string(&merged).map_err(|e| {
        redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "failed to serialize PfRedisPrice",
            e.to_string(),
        ))
    })?;

    let _: () = redis.hset("pf_prices", &merged.mint, value).await?;
    Ok(())
}

pub async fn upsert_pf_redis_bonding_curve_progress(
    state: &AppState,
    incoming: PfRedisBondingCurveProgress,
) -> Result<(), redis::RedisError> {
    let mut redis = state.cache.clone();
    let existing_json: Option<String> = redis
        .hget("pf_bonding_curve_progress", &incoming.mint)
        .await?;

    let merged = if let Some(json) = existing_json {
        let mut existing: PfRedisBondingCurveProgress =
            serde_json::from_str(&json).map_err(|e| {
                redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "failed to deserialize PfRedisBondingCurveProgress",
                    e.to_string(),
                ))
            })?;

        /* overwrite authoritative fields */
        existing.bonding_curve = incoming.bonding_curve;

        existing.virtual_sol_reserves = incoming.virtual_sol_reserves;
        existing.virtual_token_reserves = incoming.virtual_token_reserves;
        existing.real_sol_reserves = incoming.real_sol_reserves;
        existing.real_token_reserves = incoming.real_token_reserves;

        existing.progress_bps = incoming.progress_bps;
        existing.progress_pct = incoming.progress_pct;
        existing.price_lamports = incoming.price_lamports;
        existing.market_cap_lamports = incoming.market_cap_lamports;

        existing.is_pre_migration = incoming.is_pre_migration;
        existing.is_migrated = incoming.is_migrated;
        existing.is_tradeable = incoming.is_tradeable;

        existing.last_trade_slot = incoming.last_trade_slot;
        existing.last_update_ts = incoming.last_update_ts;
        existing.updated_at = Utc::now();

        existing
    } else {
        incoming
    };

    let value = serde_json::to_string(&merged).map_err(|e| {
        redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "failed to serialize PfRedisBondingCurveProgress",
            e.to_string(),
        ))
    })?;

    let _: () = redis
        .hset("pf_bonding_curve_progress", &merged.mint, value)
        .await?;

    Ok(())
}

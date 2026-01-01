use crate::models::pump_models::pf_models::pf_enums::{PfPriceSource, PfTradeDirection};
use crate::models::pump_models::pf_models::pf_kafka_event::KPfChTrade;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PfPgsqlPriceDto {
    pub mint: String,          // pre-migrated token bonding curve
    pub bonding_curve: String, // pre-migrated token bonding curve
    pub last_signature: Option<String>,
    pub price: i64,
    pub source: PfPriceSource,
    pub direction: PfTradeDirection,
    pub decimals: i16,
    pub virtual_token_reserves: i64,
    pub virtual_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub real_sol_reserves: i64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PfPgsqlBondingCurveProgressDto {
    /* ========= Identity ========= */
    pub mint: String,          // Token mint
    pub bonding_curve: String, // Bonding curve PDA
    pub last_signature: Option<String>,

    /* ========= Reserves (post-trade state) ========= */
    pub decimals: i16,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,

    /* ========= Derived progress ========= */
    pub progress_bps: u16,        // 0–10_000
    pub progress_pct: f64,        // 0.0–100.0
    pub price_lamports: u64,      // marginal price
    pub market_cap_lamports: u64, // implied market cap

    /* ========= Lifecycle flags ========= */
    pub is_pre_migration: bool,
    pub is_migrated: bool,
    pub is_tradeable: bool,
}

/// Build `PfPgsqlPriceDto` from `KPfChTrade`, validating invariants.
/// Returns `Err` if price is non-computable (protocol violation).
pub fn build_pf_pgsql_price(src: KPfChTrade) -> Result<PfPgsqlPriceDto, String> {
    // Invariant: virtual_token_reserves must never be zero
    if src.virtual_token_reserves == 0 {
        return Err("build_pf_pgsql_price: virtual_token_reserves is zero".into());
    }

    // Price calculation (decimals-aware, integer math)
    // price = (virtual_sol_reserves / virtual_token_reserves) * 10^decimals
    let scale = 10_i128
        .checked_pow(src.decimals as u32)
        .ok_or("build_pf_pgsql_price: decimals overflow")?;

    let price = (src.virtual_sol_reserves as i128)
        .checked_mul(scale)
        .ok_or("build_pf_pgsql_price: price mul overflow")?
        / src.virtual_token_reserves as i128;

    Ok(PfPgsqlPriceDto {
        mint: src.mint,
        bonding_curve: src.bonding_curve,
        last_signature: Some(src.signature),

        price: price as i64,
        source: PfPriceSource::PfTrade,
        direction: if src.is_buy {
            PfTradeDirection::Buy
        } else {
            PfTradeDirection::Sell
        },

        decimals: src.decimals,

        virtual_token_reserves: src.virtual_token_reserves as i64,
        virtual_sol_reserves: src.virtual_sol_reserves as i64,
        real_token_reserves: src.real_token_reserves as i64,
        real_sol_reserves: src.real_sol_reserves as i64,

        ts: src.ts,
    })
}

/// Build `PfPgsqlBondingCurveProgressDto` from `KPfChTrade`,
/// validating Pump.fun bonding-curve invariants.
pub fn build_pf_pgsql_bonding_curve_progress(
    src: KPfChTrade,
) -> Result<PfPgsqlBondingCurveProgressDto, String> {
    // Invariants
    if src.virtual_token_reserves == 0 {
        return Err("build_pf_pgsql_bonding_curve_progress: virtual_token_reserves is zero".into());
    }

    // Marginal price (lamports per token, decimals-aware)
    let scale = 10_u128
        .checked_pow(src.decimals as u32)
        .ok_or("build_pf_pgsql_bonding_curve_progress: decimals overflow")?;

    let price_lamports = (src.virtual_sol_reserves as u128)
        .checked_mul(scale)
        .ok_or("build_pf_pgsql_bonding_curve_progress: price overflow")?
        / src.virtual_token_reserves as u128;

    // Progress (basis points)
    // Conceptually: how much of real token supply has been consumed.
    // progress = (claimed / (claimed + remaining)) * 10_000
    let total_tokens = src
        .total_claimed_tokens
        .checked_add(src.real_token_reserves)
        .ok_or("build_pf_pgsql_bonding_curve_progress: token sum overflow")?;

    if total_tokens == 0 {
        return Err("build_pf_pgsql_bonding_curve_progress: total_tokens is zero".into());
    }

    let progress_bps = ((src.total_claimed_tokens as u128 * 10_000) / total_tokens as u128) as u16;

    let progress_pct = progress_bps as f64 / 100.0;

    // Lifecycle flags
    let is_migrated = progress_bps >= 10_000;
    let is_pre_migration = !is_migrated;
    let is_tradeable = !is_migrated;

    Ok(PfPgsqlBondingCurveProgressDto {
        /* ========= Identity ========= */
        mint: src.mint,
        bonding_curve: src.bonding_curve,
        last_signature: Some(src.signature),

        /* ========= Reserves ========= */
        decimals: src.decimals,

        virtual_sol_reserves: src.virtual_sol_reserves,
        virtual_token_reserves: src.virtual_token_reserves,
        real_sol_reserves: src.real_sol_reserves,
        real_token_reserves: src.real_token_reserves,

        /* ========= Derived progress ========= */
        progress_bps,
        progress_pct,
        price_lamports: price_lamports as u64,
        market_cap_lamports: src.market_cap_lamports,

        /* ========= Lifecycle ========= */
        is_pre_migration,
        is_migrated,
        is_tradeable,
    })
}

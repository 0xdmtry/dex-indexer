use crate::models::pump_models::pf_models::pf_enums::{PfPriceSource, PfTradeDirection};
use crate::models::pump_models::pf_models::pf_pgsql_dto::{
    PfPgsqlBondingCurveProgressDto, PfPgsqlPriceDto,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PfRedisPrice {
    pub mint: String,
    pub bonding_curve: String,

    pub name: Option<String>,
    pub symbol: Option<String>,

    pub price: i64,
    pub source: PfPriceSource,
    pub direction: PfTradeDirection,
    pub decimals: i16,
    pub virtual_token_reserves: i64,
    pub virtual_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub real_sol_reserves: i64,

    pub uri: Option<String>,
    pub description: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>,
    pub image: Option<String>,

    pub ts: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PfPgsqlPriceDto> for PfRedisPrice {
    fn from(src: PfPgsqlPriceDto) -> Self {
        let now = Utc::now();

        Self {
            mint: src.mint,
            bonding_curve: src.bonding_curve,

            // metadata is NOT available from trades
            name: None,
            symbol: None,

            price: src.price,
            source: src.source,
            direction: src.direction,
            decimals: src.decimals,

            virtual_token_reserves: src.virtual_token_reserves,
            virtual_sol_reserves: src.virtual_sol_reserves,
            real_token_reserves: src.real_token_reserves,
            real_sol_reserves: src.real_sol_reserves,

            uri: None,
            description: None,
            twitter: None,
            telegram: None,
            website: None,
            image: None,

            ts: src.ts,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PfRedisBondingCurveProgress {
    /* ========= Identity ========= */
    pub mint: String,          // Token mint
    pub bonding_curve: String, // Bonding curve PDA

    /* ========= Reserves (post-trade state) ========= */
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

    /* ========= Timing ========= */
    pub last_trade_slot: u64,
    pub last_update_ts: i64,

    pub created_at: DateTime<Utc>, // internal, creation timestamp in DB
    pub updated_at: DateTime<Utc>, // internal, last update timestamp
}

impl From<PfPgsqlBondingCurveProgressDto> for PfRedisBondingCurveProgress {
    fn from(src: PfPgsqlBondingCurveProgressDto) -> Self {
        let now = Utc::now();

        Self {
            /* ========= Identity ========= */
            mint: src.mint,
            bonding_curve: src.bonding_curve,

            /* ========= Reserves ========= */
            virtual_sol_reserves: src.virtual_sol_reserves,
            virtual_token_reserves: src.virtual_token_reserves,
            real_sol_reserves: src.real_sol_reserves,
            real_token_reserves: src.real_token_reserves,

            /* ========= Derived progress ========= */
            progress_bps: src.progress_bps,
            progress_pct: src.progress_pct,
            price_lamports: src.price_lamports,
            market_cap_lamports: src.market_cap_lamports,

            /* ========= Lifecycle ========= */
            is_pre_migration: src.is_pre_migration,
            is_migrated: src.is_migrated,
            is_tradeable: src.is_tradeable,

            /* ========= Timing ========= */
            last_trade_slot: 0,              // not available here
            last_update_ts: now.timestamp(), // wall-clock update

            created_at: now,
            updated_at: now,
        }
    }
}

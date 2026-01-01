use crate::models::pump_models::pf_models::pf_enums::{PfPriceSource, PfTradeDirection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PfPgsqlPrice {
    pub id: i64,               // internal DB identifier
    pub mint: String,          // pre-migrated token bonding curve
    pub bonding_curve: String, // pre-migrated token bonding curve
    pub last_signature: Option<String>,
    pub price: i64,
    pub source: PfPriceSource,
    pub direction: PfTradeDirection,
    pub decimal: i16,
    pub virtual_token_reserves: i64,
    pub virtual_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub real_sol_reserves: i64,
    pub ts: DateTime<Utc>,
    pub created_at: DateTime<Utc>, // internal, creation timestamp in DB
    pub updated_at: DateTime<Utc>, // internal, last update timestamp
}

#[derive(Debug, Clone)]
pub struct PfPgsqlBondingCurveProgress {
    /* ========= Identity ========= */
    pub mint: String,          // Token mint
    pub bonding_curve: String, // Bonding curve PDA
    pub last_signature: Option<String>,

    /* ========= Reserves (post-trade state) ========= */
    pub decimal: i16,
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

    pub created_at: DateTime<Utc>, // internal, creation timestamp in DB
    pub updated_at: DateTime<Utc>, // internal, last update timestamp
}

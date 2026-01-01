use crate::models::enums::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenWithPools {
    pub token: Token,
    pub pools: Vec<Pool>,
}

// TODO Unified structure of Token, KToken, EnrichedResolvedToken, and KTokenReqFulfill
/// Stores the foundational static metadata for a token, captured at creation.
/// Type: Raw data (parsed).
/// Source: Parsed from Geyser logs (TxConsumer "Create" event).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i64,             // internal DB identifier
    pub name: String,        // token name
    pub symbol: String,      // token symbol
    pub uri: Option<String>, // metadata URI
    pub creator: String,     // wallet that deployed the token
    pub mint: String,        // token mint
    pub platform: Platform,

    pub bonding_curve: Option<String>, // bonding curve PDA
    pub pool_state: Option<String>,
    pub pool: Option<String>,

    pub user_address: String, // wallet that executed the creator
    pub decimal: i64,
    pub ts: DateTime<Utc>,           // block timestamp
    pub virtual_token_reserves: i64, // bonding curve virtual token reserves
    pub virtual_sol_reserves: i64,   // bonding curve virtual SOL reserves
    pub real_token_reserves: i64,    // actual token reserves
    pub token_total_supply: i64,     // total token supply

    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
    pub image: String,

    pub created_at: DateTime<Utc>, // internal, creation timestamp in DB
    pub updated_at: DateTime<Utc>, // internal, last update timestamp
}

/// Stores discovered DEX liquidity pools from various platforms.
/// Type: Raw data (discovered).
/// Source: Parsed from Geyser logs (TxConsumer/PoolConsumer) or RPC Backfill.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pool {
    pub id: i64,                   // internal DB identifier
    pub mint: String,              // token mint this pool is linked to
    pub platform: Platform,        // source platform (Raydium, Meteora, PumpSwap)
    pub pool: String,              // actual on-chain pool address
    pub pair: String,              // trading pair (e.g. SOL/USDC)
    pub token_a: String,           // base token mint
    pub token_b: String,           // quote token mint
    pub created_at: DateTime<Utc>, // internal, creation timestamp in DB
    pub updated_at: DateTime<Utc>, // internal, last update timestamp
}

use crate::models::enums::Platform;
use crate::models::kafka_event::{TokenStatus, TradeDirection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload containing the foundational metadata of a newly created token.
/// Purpose: To register a new token in the system, captured from its "Create" event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfToken {
    pub mint: String,          // token mint
    pub bonding_curve: String, // bonding curve PDA
    pub name: String,          // token name
    pub symbol: String,        // token symbol
    pub uri: Option<String>,   // metadata URI
    pub creator: String,       // wallet that deployed the token
    pub user_address: String,  // wallet that executed the creator
    pub decimals: i16,
    pub ts: DateTime<Utc>, // block timestamp
    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
    pub image: String,
    pub virtual_token_reserves: i64, // bonding curve virtual token reserves
    pub virtual_sol_reserves: i64,   // bonding curve virtual SOL reserves
    pub real_token_reserves: i64,    // actual token reserves
    pub token_total_supply: i64,     // total token supply
}

/// Kafka payload for an individual swap event (a buy or sell).
/// Purpose: To log a single trade, which is used to feed all downstream aggregations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfTrade {
    pub signature: String,         // transaction signature
    pub mint: String,              // token mint being traded
    pub bonding_curve: String,     // bonding curve PDA
    pub platform: Platform,        // PumpSwap / Raydium / Meteora
    pub direction: TradeDirection, //`Buy` or `Sell`
    pub sol_amount: i64,           // amount of SOL (in lamports) exchanged
    pub token_amount: i64,         // amount of token (in smallest unit) exchanged
    pub user_pubkey: String,       // wallet that executed the swap
    pub ts: DateTime<Utc>,         // transaction time
}

/// Kafka payload representing a change in a token's lifecycle state.
/// Purpose: To either log the initial creation or (more often) update a token to "Migrated".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfTokenLifecycle {
    pub mint: String,                       // wallet that created token
    pub status: TokenStatus,                // CREATED / MIGRATED
    pub platform: Platform,                 // PumpSwap / Raydium / Meteora
    pub ts_created: Option<DateTime<Utc>>,  // time token was created
    pub ts_migrated: Option<DateTime<Utc>>, // time token migrated
    pub bonding_curve: Option<String>,      // bonding_curve
    pub pool: Option<String>,               // destination pool after migration (from Migrate event)
    pub sol_amount_migrated: Option<i64>, // SOL transferred during migration (lamports, from Migrate event)
    pub token_amount_migrated: Option<i64>, // tokens transferred during migration (from Migrate event)
}

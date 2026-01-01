use crate::models::pump_models::ps_models::ps_enums::{PsPriceSource, PsTradeDirection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload for a single, discrete price event.
/// Purpose: To log a new price point for a token from any monitored source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPsPrice {
    pub pool: String,          // token pool
    pub source: PsPriceSource, // source of this specific price event
    pub ts: DateTime<Utc>,     // on-chain timestamp (block time) of the event
    pub token_a_reserves: i64,
    pub token_b_reserves: i64,
}

/// Kafka payload containing the foundational metadata of a newly created token.
/// Purpose: To register a new token in the system, captured from its "Create" event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPsToken {
    pub mint: String,         // token mint
    pub pool: String,         // pool PDA
    pub name: String,         // token name
    pub symbol: String,       // token symbol
    pub creator: String,      // wallet that deployed the token
    pub user_address: String, // wallet that executed the creator
    pub decimals: i16,
    pub ts: DateTime<Utc>,   // block timestamp
    pub uri: Option<String>, // metadata URI
    pub description: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String,
    pub image: String,
}

/// Kafka payload for an individual swap event (a buy or sell).
/// Purpose: To log a single trade, which is used to feed all downstream aggregations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPsTrade {
    pub signature: String,           // transaction signature
    pub mint: String,                // token mint being traded
    pub pool: String,                // bonding curve PDA
    pub direction: PsTradeDirection, //`Buy` or `Sell`
    pub sol_amount: i64,             // amount of SOL (in lamports) exchanged
    pub token_amount: i64,           // amount of token (in smallest unit) exchanged
    pub user_pubkey: String,         // wallet that executed the swap
    pub ts: DateTime<Utc>,           // transaction time
}

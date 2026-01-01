use crate::models::kafka_event::TradeDirection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload for an individual swap event (a buy or sell).
/// Purpose: To log a single trade, which is used to feed all downstream aggregations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPsTrade {
    pub signature: String,         // transaction signature
    pub mint: String,              // token mint being traded
    pub pool: String,              // bonding curve PDA
    pub direction: TradeDirection, //`Buy` or `Sell`
    pub sol_amount: i64,           // amount of SOL (in lamports) exchanged
    pub token_amount: i64,         // amount of token (in smallest unit) exchanged
    pub user_pubkey: String,       // wallet that executed the swap
    pub ts: DateTime<Utc>,         // transaction time
}

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

use crate::models::enums::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload containing the foundational metadata of a newly created token.
/// Purpose: To register a new token in the system, captured from its "Create" event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRllToken {
    pub name: String,        // token name
    pub symbol: String,      // token symbol
    pub uri: Option<String>, // metadata URI
    pub creator: String,     // wallet that deployed the token
    pub mint: String,        // token mint
    pub platform: Platform,

    pub pool_state: String, // bonding curve PDA

    pub user_address: String, // wallet that executed the creator

    pub ts: DateTime<Utc>, // block timestamp

    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub virtual_base: u64,
    pub virtual_quote: u64,
}

/// Kafka payload for an individual swap event (a buy or sell).
/// Purpose: To log a single trade, which is used to feed all downstream aggregations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRllTrade {}

/// Kafka payload representing a change in a token's lifecycle state.
/// Purpose: To either log the initial creation or (more often) update a token to "Migrated".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRllTokenLifecycle {}

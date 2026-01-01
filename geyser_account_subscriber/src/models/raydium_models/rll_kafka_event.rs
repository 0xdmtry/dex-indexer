use crate::models::kafka_event::PriceSource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload for a single, discrete price event.
/// Purpose: To log a new price point for a token from any monitored source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRllPrice {
    pub pool_state: String,  // token pool state
    pub source: PriceSource, // source of this specific price event
    pub ts: DateTime<Utc>,   // on-chain timestamp (block time) of the event

    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub virtual_base: u64,
    pub virtual_quote: u64,
}

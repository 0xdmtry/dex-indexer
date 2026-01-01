use crate::models::kafka_event::PriceSource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload for a single, discrete price event.
/// Purpose: To log a new price point for a token from any monitored source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPsPrice {
    pub pool: String,        // token pool
    pub source: PriceSource, // source of this specific price event
    pub ts: DateTime<Utc>,   // on-chain timestamp (block time) of the event

    pub token_a_reserves: u64,
    pub token_b_reserves: u64,
}

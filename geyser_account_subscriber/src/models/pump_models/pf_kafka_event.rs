use crate::models::kafka_event::PriceSource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Kafka payload for a single, discrete price event.
/// Purpose: To log a new price point for a token from any monitored source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPfPrice {
    pub bonding_curve: String, // token bonding curve
    pub source: PriceSource,   // source of this specific price event
    pub ts: DateTime<Utc>,     // on-chain timestamp (block time) of the event

    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
}

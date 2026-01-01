use crate::models::pump_models::pf_ch_kafka_event::PfChTradeUnified;
use crate::models::pump_models::pf_kafka_event::{KPfToken, KPfTokenLifecycle, KPfTrade};
use crate::models::pump_models::ps_kafka_event::{KPsToken, KPsTrade};
use crate::models::raydium_models::rll_kafka_event::{KRllToken, KRllTokenLifecycle, KRllTrade};
use serde::{Deserialize, Serialize};

/// Represents the lifecycle state of a token.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TokenStatus {
    Created,
    Migrated,
}

/// Represents the direction of a swap
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}

/// Enumerates the different types of events that can be sent over Kafka.
/// This acts as a routing key for the consumer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KEventType {
    PfChTradeOccurred,

    PfTradeOccurred,
    PfTokenCreated,
    PfTokenMigrated,

    PsTradeOccurred,

    RllTradeOccurred,
    RllTokenCreated,
    RllTokenMigrated,
}

/// Represents the data payload for a specific Kafka event.
/// Each variant holds the struct corresponding to its event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KEventData {
    PfChTradeOccurred(PfChTradeUnified),

    PfTradeOccurred(KPfTrade),
    PfTokenCreated(KPfToken),
    PfTokenMigrated(KPfTokenLifecycle),

    PsTradeOccurred(KPsTrade),
    PsTokenCreated(KPsToken),

    RllTradeOccurred(KRllTrade),
    RllTokenCreated(KRllToken),
    RllTokenMigrated(KRllTokenLifecycle),
}

/// The top-level structure for a message sent from the Producer API over Kafka.
/// It contains a type identifier and the corresponding data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KEvent {
    /// The type of event, used by the consumer to determine how to process the payload.
    pub event_type: KEventType,

    /// The actual data payload, which varies depending on the `event_type`.
    pub data: KEventData,
}

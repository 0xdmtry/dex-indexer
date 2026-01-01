use crate::models::pump_models::pf_models::pf_kafka_event::{
    KPfChTrade, KPfPrice, KPfToken, KPfTokenLifecycle, KPfTrade,
};
use crate::models::pump_models::ps_models::ps_kafka_event::{KPsPrice, KPsToken, KPsTrade};
use serde::{Deserialize, Serialize};

/// Enumerates the different types of events that can be sent over Kafka.
/// This acts as a routing key for the consumer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KEventType {
    PfChTradeOccurred,

    PfTradeOccurred,
    PfTokenCreated,
    PfTokenMigrated,
    PfPriceUpdated,

    PsTradeOccurred,
    PsTokenCreated,
    PsPriceUpdated,
}

/// Represents the data payload for a specific Kafka event.
/// Each variant holds the struct corresponding to its event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KEventData {
    PfChTradeOccurred(KPfChTrade),

    PfTradeOccurred(KPfTrade),
    PfTokenCreated(KPfToken),
    PfTokenMigrated(KPfTokenLifecycle),
    PfPriceUpdated(KPfPrice),

    PsTradeOccurred(KPsTrade),
    PsTokenCreated(KPsToken),
    PsPriceUpdated(KPsPrice),
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

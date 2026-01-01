use crate::models::pump_models::pf_kafka_event::KPfPrice;
use crate::models::pump_models::ps_kafka_event::KPsPrice;
use crate::models::raydium_models::rll_kafka_event::KRllPrice;
use serde::{Deserialize, Serialize};

/// Represents the origin of a specific price data point.
/// This corresponds to the event source that generated the price.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceSource {
    PumpFunBondingCurve,
    PumpSwapPool,
    RaydiunmLaunchLabPoolState,
}

/// Enumerates the different types of events that can be sent over Kafka.
/// This acts as a routing key for the consumer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KEventType {
    PfPriceUpdated,
    PsPriceUpdated,
    RllPriceUpdated,
}

/// Represents the data payload for a specific Kafka event.
/// Each variant holds the struct corresponding to its event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)] // Allows deserializing into the correct variant based on field matching
pub enum KEventData {
    PfPriceUpdated(KPfPrice),
    PsPriceUpdated(KPsPrice),
    RllPriceUpdated(KRllPrice),
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

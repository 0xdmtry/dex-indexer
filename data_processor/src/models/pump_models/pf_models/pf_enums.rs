use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "price_source")]
#[sqlx(rename_all = "snake_case")]
pub enum PfPriceSource {
    PfBondingCurve,
    PfTrade,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "trade_direction")]
#[sqlx(rename_all = "snake_case")]
pub enum PfTradeDirection {
    Buy,
    Sell,
}

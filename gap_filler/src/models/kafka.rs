use crate::models::enums::{KReqData, KReqType, Platform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPriceReqBondingCurve {
    pub bonding_curves: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPriceReqPool {
    pub pools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPriceReqPoolState {
    pub pools_states: Vec<String>,
}

// TODO Unified structure of Token, KToken, EnrichedResolvedToken, and KTokenReqFulfill
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KTokenReqFulfill {
    pub mint: String,
    pub platform: Platform,

    pub bonding_curve: Option<String>,
    pub pool: Option<String>,
    pub pool_state: Option<String>,

    pub price: i64,
    pub decimal: u8,

    // ResolvedTokenMetadata
    pub name: String,
    pub symbol: String,
    pub uri: String,

    // ExtendedMetadata
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub twitter: String,
    #[serde(default)]
    pub telegram: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KReq {
    pub req_type: KReqType,
    pub platform: Platform,
    pub data: KReqData,
}

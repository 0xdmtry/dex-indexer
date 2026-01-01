use crate::models::enums::Platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedResolvedToken {
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

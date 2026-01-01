use crate::models::enums::Platform;
use crate::models::rpc::PriceError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Cache error: {0}")]
    Cache(#[from] crate::models::cache::CacheError),
    #[error("Database error: {0}")]
    Db(#[from] crate::models::db::DbError),
    #[error("Price error: {0}")]
    Price(#[from] PriceError),
    #[error("Timeout")]
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedToken {
    pub mint: String,
    pub bonding_curve: Option<String>,
    pub pool: Option<String>,
    pub price: i64,
}

#[derive(Debug, Clone, Default)]
pub struct ResolvedTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExtendedMetadata {
    pub name: String,
    pub symbol: String,
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

// TODO Unified structure of Token, KToken, EnrichedResolvedToken, and KTokenReqFulfillv
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

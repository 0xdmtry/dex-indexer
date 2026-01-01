use crate::models::kafka::{
    KPriceReqBondingCurve, KPriceReqPool, KPriceReqPoolState, KTokenReqFulfill,
};
use crate::models::rpc::{BondingCurveAccount, PoolAccount, PoolState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KReqType {
    PriceReqBondingCurve,
    PriceReqPool,
    PriceReqPoolState,
    TokenReqFulfill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KReqData {
    PriceReqBondingCurve(KPriceReqBondingCurve),
    PriceReqPool(KPriceReqPool),
    PriceReqPoolState(KPriceReqPoolState),
    TokenReqFulfill(Box<KTokenReqFulfill>),
}

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "platform_type", rename_all = "snake_case")]
pub enum Platform {
    // Pump Platforms
    PumpFun,
    PumpSwap,

    // Raydium Pool Types
    RaydiumAmmV4,
    RaydiumCpmm,
    RaydiumClmm,
    RaydiumLaunchLab,

    // Meteora Pool Types
    MeteoraDlmm,
    MeteoraDlmmLaunch,
    MeteoraDammV1,
    MeteoraDammV2,
    MeteoraMemecoinV1,
    MeteoraMemecoinV2,
    MeteoraStake2Earn,
    MeteoraDbc,

    // Fallback
    Unknown,
}

/////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcAccountType {
    PumpFunToken2022,
    PumpFunSpl,
    PumpSwap,
    RaydiumLaunchLab,
}

#[derive(Debug, Clone)]
pub enum RpcAccountData {
    PumpFunToken2022(BondingCurveAccount),
    PumpFunSpl(BondingCurveAccount),
    PumpSwap(PoolAccount),
    RaydiumLaunchLab(PoolState),
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

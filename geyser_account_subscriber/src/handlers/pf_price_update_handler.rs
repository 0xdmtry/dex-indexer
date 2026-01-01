use crate::models::kafka_event::{KEvent, KEventData, KEventType, PriceSource};
use crate::models::pump_models::pf_kafka_event::KPfPrice;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use solana_sdk::bs58;
use yellowstone_grpc_proto::prelude::SubscribeUpdateAccountInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BondingCurveData {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
}

pub fn handle_pf_price_update(acc_info: &SubscribeUpdateAccountInfo) -> Result<KEvent, String> {
    let curve = parse_bonding_curve(acc_info).ok_or("Failed to parse bonding curve")?;
    let bonding_curve = bs58::encode(&acc_info.pubkey).into_string();
    let k_pf_price: KPfPrice = KPfPrice {
        bonding_curve,
        source: PriceSource::PumpFunBondingCurve,
        ts: Utc::now(),
        virtual_token_reserves: curve.virtual_token_reserves,
        virtual_sol_reserves: curve.virtual_sol_reserves,
        real_token_reserves: curve.real_token_reserves,
        real_sol_reserves: curve.real_sol_reserves,
    };

    Ok(KEvent {
        event_type: KEventType::PfPriceUpdated,
        data: KEventData::PfPriceUpdated(k_pf_price),
    })
}

fn parse_bonding_curve(account_info: &SubscribeUpdateAccountInfo) -> Option<BondingCurveData> {
    if account_info.data.len() < 40 {
        return None;
    }

    let data = &account_info.data;

    Some(BondingCurveData {
        virtual_token_reserves: u64::from_le_bytes(data[8..16].try_into().ok()?),
        virtual_sol_reserves: u64::from_le_bytes(data[16..24].try_into().ok()?),
        real_token_reserves: u64::from_le_bytes(data[24..32].try_into().ok()?),
        real_sol_reserves: u64::from_le_bytes(data[32..40].try_into().ok()?),
    })
}

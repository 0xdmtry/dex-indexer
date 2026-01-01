use crate::models::kafka_event::{KEvent, KEventData, KEventType, PriceSource};
use crate::models::pump_models::ps_kafka_event::KPsPrice;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use solana_sdk::bs58;
use yellowstone_grpc_proto::prelude::SubscribeUpdateAccountInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PumpSwapPoolData {
    pub token_a_reserves: u64,
    pub token_b_reserves: u64,
}

pub fn handle_ps_price_update(acc_info: &SubscribeUpdateAccountInfo) -> Result<KEvent, String> {
    let pool_data = parse_pool(acc_info).ok_or("Failed to parse pool")?;
    let pool = bs58::encode(&acc_info.pubkey).into_string();
    let k_ps_price: KPsPrice = KPsPrice {
        pool,
        source: PriceSource::PumpSwapPool,
        ts: Utc::now(),
        token_a_reserves: pool_data.token_a_reserves,
        token_b_reserves: pool_data.token_b_reserves,
    };

    Ok(KEvent {
        event_type: KEventType::PsPriceUpdated,
        data: KEventData::PsPriceUpdated(k_ps_price),
    })
}

fn parse_pool(account_info: &SubscribeUpdateAccountInfo) -> Option<PumpSwapPoolData> {
    if account_info.data.len() < 24 {
        return None;
    }

    let data = &account_info.data;

    Some(PumpSwapPoolData {
        token_a_reserves: u64::from_le_bytes(data[8..16].try_into().ok()?),
        token_b_reserves: u64::from_le_bytes(data[16..24].try_into().ok()?),
    })
}

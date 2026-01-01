use crate::models::kafka_event::{KEvent, KEventData, KEventType, PriceSource};
use crate::models::raydium_models::rll_kafka_event::KRllPrice;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use solana_sdk::bs58;
use yellowstone_grpc_proto::prelude::SubscribeUpdateAccountInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PoolStateData {
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub virtual_base: u64,
    pub virtual_quote: u64,
}

pub fn handle_rll_price_update(acc_info: &SubscribeUpdateAccountInfo) -> Result<KEvent, String> {
    let pool_state = bs58::encode(&acc_info.pubkey).into_string();
    let ps = extract_price_data(acc_info);
    let k_rll_price: KRllPrice = KRllPrice {
        pool_state,
        source: PriceSource::RaydiunmLaunchLabPoolState,
        ts: Utc::now(),
        base_decimals: ps.base_decimals,
        quote_decimals: ps.quote_decimals,
        virtual_base: ps.virtual_base,
        virtual_quote: ps.virtual_quote,
    };

    Ok(KEvent {
        event_type: KEventType::PfPriceUpdated,
        data: KEventData::RllPriceUpdated(k_rll_price),
    })
}

fn extract_price_data(account_info: &SubscribeUpdateAccountInfo) -> PoolStateData {
    let data = &account_info.data;

    // TODO properly calculate necessary fields
    PoolStateData {
        base_decimals: data[18],
        quote_decimals: data[19],
        virtual_base: 1_000_000, /*u64::from_le_bytes([
                                     data[38], data[39], data[40], data[41], data[42], data[43], data[44], data[45],
                                 ])*/
        virtual_quote: 1_000_000, /*u64::from_le_bytes([
                                      data[46], data[47], data[48], data[49], data[50], data[51], data[52], data[53],
                                  ])*/
    }
}

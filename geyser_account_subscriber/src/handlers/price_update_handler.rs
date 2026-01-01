use crate::handlers::pf_price_update_handler::handle_pf_price_update;
use crate::handlers::ps_price_update_handler::handle_ps_price_update;
use crate::handlers::rll_price_update_handler::handle_rll_price_update;
use crate::models::enums::Platform;
use crate::models::kafka_event::KEvent;
use solana_sdk::bs58;
use std::collections::HashMap;
use yellowstone_grpc_proto::prelude::SubscribeUpdateAccountInfo;

pub async fn handle_price_update(
    acc_info: &SubscribeUpdateAccountInfo,
    tracked_accounts: HashMap<String, Platform>,
) -> Result<KEvent, String> {
    let identifier = bs58::encode(&acc_info.pubkey).into_string();
    let platform = get_platform(&identifier, &tracked_accounts).await;

    match platform {
        Some(Platform::PumpFun) => handle_pf_price_update(acc_info),
        Some(Platform::PumpSwap) => handle_ps_price_update(acc_info),
        Some(Platform::RaydiumLaunchLab) => handle_rll_price_update(acc_info),
        _ => Err("Unknown or unsupported platform".to_string()),
    }
}

pub async fn get_platform(
    key: &str,
    tracked_accounts: &HashMap<String, Platform>,
) -> Option<Platform> {
    tracked_accounts.get(key).cloned()
}

use crate::handlers::pumpfun::pf_tx_handler::handle_pf_tx;
use crate::handlers::pumpswap::ps_tx_handler::handle_ps_tx;
use crate::handlers::raydium_launchlab::rll_tx_handler::handle_rll_tx;
use crate::models::consts::{
    PUMPFUN_PROGRAM_ID, PUMPSWAP_PROGRAM_ID, RAYDIUM_LAUNCHLAB_PROGRAM_ID,
};
use crate::models::enums::Platform;
use crate::models::kafka_event::KEvent;
use log::error;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_tx(
    tx_info: SubscribeUpdateTransactionInfo,
    slot: u64,
) -> Result<Option<KEvent>, String> {
    let platform = get_platform(&tx_info);

    match platform {
        Some(p) => match p {
            Platform::PumpFun => handle_pf_tx(tx_info, slot),
            Platform::PumpSwap => handle_ps_tx(tx_info),
            Platform::RaydiumLaunchLab => handle_rll_tx(tx_info),
        },
        None => {
            let err = "Unknown platform".to_string();
            error!("{err}");
            Err(err)
        }
    }
}

fn get_platform(tx_info: &SubscribeUpdateTransactionInfo) -> Option<Platform> {
    get_platform_from_instructions(tx_info)
        .or_else(|| get_platform_from_logs(tx_info))
        .or_else(|| get_platform_from_account_keys(tx_info))
        .or_else(|| get_platform_from_inner_instructions(tx_info))
}

fn program_id_to_platform(program_id_bytes: &[u8]) -> Option<Platform> {
    let pubkey = Pubkey::try_from(program_id_bytes).ok()?;
    let program_id = pubkey.to_string();

    match program_id.as_str() {
        s if s == PUMPFUN_PROGRAM_ID => Some(Platform::PumpFun),
        s if s == PUMPSWAP_PROGRAM_ID => Some(Platform::PumpSwap),
        s if s == RAYDIUM_LAUNCHLAB_PROGRAM_ID => Some(Platform::RaydiumLaunchLab),
        _ => None,
    }
}

fn get_platform_from_account_keys(tx_info: &SubscribeUpdateTransactionInfo) -> Option<Platform> {
    let program_id = tx_info
        .transaction
        .as_ref()?
        .message
        .as_ref()?
        .account_keys
        .first()?;
    program_id_to_platform(program_id)
}

fn get_platform_from_instructions(tx_info: &SubscribeUpdateTransactionInfo) -> Option<Platform> {
    let msg = tx_info.transaction.as_ref()?.message.as_ref()?;
    let instruction = msg.instructions.first()?;

    let program_id_index = instruction.program_id_index as usize;
    let program_id = msg.account_keys.get(program_id_index)?;

    program_id_to_platform(program_id)
}

fn get_platform_from_logs(tx_info: &SubscribeUpdateTransactionInfo) -> Option<Platform> {
    let logs = &tx_info.meta.as_ref()?.log_messages;

    const TARGETS: &[(&str, Platform)] = &[
        (PUMPSWAP_PROGRAM_ID, Platform::PumpSwap),
        (PUMPFUN_PROGRAM_ID, Platform::PumpFun),
        (RAYDIUM_LAUNCHLAB_PROGRAM_ID, Platform::RaydiumLaunchLab),
    ];

    for (program_id, platform) in TARGETS {
        if logs.iter().any(|log| log.contains(program_id)) {
            return Some(*platform);
        }
    }

    None
}

fn get_platform_from_inner_instructions(
    tx_info: &SubscribeUpdateTransactionInfo,
) -> Option<Platform> {
    let msg = tx_info.transaction.as_ref()?.message.as_ref()?;
    let inner = tx_info.meta.as_ref()?.inner_instructions.first()?;
    let instruction = inner.instructions.first()?;
    let program_id_index = instruction.program_id_index as usize;
    let program_id = msg.account_keys.get(program_id_index)?;
    program_id_to_platform(program_id)
}

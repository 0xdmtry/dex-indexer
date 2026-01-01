use crate::handlers::pumpfun::pf_ch_trade_occurred_handler::handle_pf_ch_trade;
use crate::models::kafka_event::{KEvent, KEventData, KEventType};
use log::error;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_pf_tx(
    tx_info: SubscribeUpdateTransactionInfo,
    slot: u64,
) -> Result<Option<KEvent>, String> {
    let logs = tx_info
        .clone()
        .meta
        .map(|m| m.log_messages)
        .unwrap_or_default();

    if is_pf_trade(&logs) {
        match handle_pf_ch_trade(&tx_info, slot) {
            Ok(pf_ch_trade) => Ok(Some(KEvent {
                event_type: KEventType::PfChTradeOccurred,
                data: KEventData::PfChTradeOccurred(pf_ch_trade),
            })),
            Err(e) => {
                let err = format!("Failed to handle PF trade: {e}");
                error!("{err}");
                Err(err)
            }
        }
    } else if is_pf_create(&logs) {
        return Ok(None);
        // match handle_pf_token_creation(&tx_info) {
        //     Ok(ktoken) => Ok(Some(KEvent {
        //         event_type: KEventType::PfTokenCreated,
        //         data: KEventData::PfTokenCreated(ktoken),
        //     })),
        //     Err(e) => {
        //         let err = format!("Failed to handle token creation: {e}");
        //         error!("{err}");
        //         Err(err)
        //     }
        // }
    } else if is_pf_migrate(&logs) {
        return Ok(None);
        // match handle_pf_token_migration(&tx_info) {
        //     Ok(ktoken_lifecycle) => Ok(Some(KEvent {
        //         event_type: KEventType::PfTokenMigrated,
        //         data: KEventData::PfTokenMigrated(ktoken_lifecycle),
        //     })),
        //     Err(e) => {
        //         let err = format!("Failed to handle migration event: {e}");
        //         error!("{err}");
        //         Err(err)
        //     }
        // }
    } else {
        Ok(None)
    }
}

fn is_pf_create(logs: &[String]) -> bool {
    if logs
        .iter()
        .any(|log| log.contains("Program log: Instruction: CreateV2"))
    {
        return true;
    }

    false
}

fn is_pf_migrate(logs: &[String]) -> bool {
    if logs
        .iter()
        .any(|log| log.contains("Program log: Instruction: Migrate"))
    {
        return true;
    }

    false
}

fn is_pf_trade(logs: &[String]) -> bool {
    if logs.iter().any(|log| {
        log.starts_with("Program log: SwapEvent")
            || log.contains("Program log: Instruction: Buy")
            || log.contains("Program log: Instruction: Sell")
    }) {
        return true;
    }
    false
}

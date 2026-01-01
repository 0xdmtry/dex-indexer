use crate::handlers::raydium_launchlab::rll_token_created_handler::handle_rll_token_creation;
use crate::handlers::raydium_launchlab::rll_token_migrated_handler::handle_rll_token_migration;
use crate::handlers::raydium_launchlab::rll_trade_occurred_handler::handle_rll_trade;
use crate::models::kafka_event::{KEvent, KEventData, KEventType};
use log::error;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_rll_tx(tx_info: SubscribeUpdateTransactionInfo) -> Result<Option<KEvent>, String> {
    let logs = tx_info
        .clone()
        .meta
        .map(|m| m.log_messages)
        .unwrap_or_default();

    if is_rll_trade(&logs) {
        match handle_rll_trade(&tx_info) {
            Ok(ktrade) => Ok(Some(KEvent {
                event_type: KEventType::PfTradeOccurred,
                data: KEventData::PfTradeOccurred(ktrade),
            })),
            Err(e) => {
                let err = format!("Failed to handle RLL trade: {e}");
                error!("{err}");
                Err(err)
            }
        }
    } else if is_rll_create(&logs) {
        match handle_rll_token_creation(&tx_info) {
            Ok(ktoken) => Ok(Some(KEvent {
                event_type: KEventType::PfTokenCreated,
                data: KEventData::PfTokenCreated(ktoken),
            })),
            Err(e) => {
                let err = format!("Failed to handle token creation: {e}");
                error!("{err}");
                Err(err)
            }
        }
    } else if is_rll_migrate(&logs) {
        match handle_rll_token_migration(&tx_info) {
            Ok(ktoken_lifecycle) => Ok(Some(KEvent {
                event_type: KEventType::PfTokenMigrated,
                data: KEventData::PfTokenMigrated(ktoken_lifecycle),
            })),
            Err(e) => {
                let err = format!("Failed to handle migration event: {e}");
                error!("{err}");
                Err(err)
            }
        }
    } else {
        Ok(None)
    }
}

fn is_rll_create(logs: &[String]) -> bool {
    if logs
        .iter()
        .any(|log| log.contains("Program log: Instruction: CreateV2"))
    {
        return true;
    }

    false
}

fn is_rll_migrate(logs: &[String]) -> bool {
    if logs
        .iter()
        .any(|log| log.contains("Program log: Instruction: Migrate"))
    {
        return true;
    }

    false
}

fn is_rll_trade(logs: &[String]) -> bool {
    if logs.iter().any(|log| {
        log.starts_with("Program log: SwapEvent")
            || log.contains("Program log: Instruction: Buy")
            || log.contains("Program log: Instruction: Sell")
    }) {
        return true;
    }
    false
}

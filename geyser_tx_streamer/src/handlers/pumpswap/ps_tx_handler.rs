use crate::handlers::pumpswap::ps_trade_occurred_handler::handle_ps_trade;
use crate::models::kafka_event::{KEvent, KEventData, KEventType};
use log::error;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_ps_tx(tx_info: SubscribeUpdateTransactionInfo) -> Result<Option<KEvent>, String> {
    let logs = tx_info
        .clone()
        .meta
        .map(|m| m.log_messages)
        .unwrap_or_default();

    if is_ps_trade(&logs) {
        match handle_ps_trade(&tx_info) {
            Ok(ktrade) => Ok(Some(KEvent {
                event_type: KEventType::PsTradeOccurred,
                data: KEventData::PsTradeOccurred(ktrade),
            })),
            Err(e) => {
                let err = format!("handle_ps_tx: Failed to handle PS trade: {e}");
                error!("{err}");
                Err(err)
            }
        }
    } else {
        Ok(None)
    }
}

fn is_ps_trade(logs: &[String]) -> bool {
    if logs.iter().any(|log| {
        log.starts_with("Program log: SwapEvent")
            || log.contains("Program log: Instruction: Buy")
            || log.contains("Program log: Instruction: Sell")
    }) {
        return true;
    }
    false
}

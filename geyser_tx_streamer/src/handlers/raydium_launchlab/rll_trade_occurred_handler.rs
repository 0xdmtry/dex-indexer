use crate::models::pump_models::pf_kafka_event::KPfTrade;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_rll_trade(_tx_info: &SubscribeUpdateTransactionInfo) -> Result<KPfTrade, String> {
    todo!();
}

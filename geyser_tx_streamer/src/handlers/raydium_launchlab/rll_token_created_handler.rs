use crate::models::pump_models::pf_kafka_event::KPfToken;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;

pub fn handle_rll_token_creation(
    _tx_info: &SubscribeUpdateTransactionInfo,
) -> Result<KPfToken, String> {
    todo!();
}

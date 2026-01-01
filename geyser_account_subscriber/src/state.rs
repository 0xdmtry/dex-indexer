use crate::api::geyser::subscription_manager::SubscriptionManagerHandle;
use crate::config::AppConfig;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub subscription_handle: SubscriptionManagerHandle,
    pub cache: ConnectionManager,
}
pub fn init_state(
    config: AppConfig,
    subscription_handle: SubscriptionManagerHandle,
    cache: ConnectionManager,
) -> AppState {
    AppState {
        config,
        subscription_handle,
        cache,
    }
}

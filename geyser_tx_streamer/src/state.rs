use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

pub fn init_state(config: AppConfig) -> AppState {
    AppState { config }
}

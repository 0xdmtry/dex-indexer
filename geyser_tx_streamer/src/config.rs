use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub geyser_url: String,
    pub geyser_token: Option<String>,
    pub kafka_brokers: String,
    pub kafka_group_id: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename_override(".env").ok();

        let geyser_url = env::var("GEYSER_URL").expect("GEYSER_URL must be set in .env");
        let geyser_token = env::var("GEYSER_TOKEN").expect("GEYSER_TOKEN must be set in .env");
        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "kafka:9092".to_string());
        let kafka_group_id =
            env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "pump_data_producer".to_string());

        Self {
            geyser_url,
            geyser_token: Some(geyser_token),
            kafka_brokers,
            kafka_group_id,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename_override(file_name).ok();
        Self::from_env()
    }
}

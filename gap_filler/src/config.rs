use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub pg_url: String,
    pub rpc_http_url: String,
    pub kafka_brokers: String,
    pub kafka_group_id: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename_override(".env").ok();

        let pg_url =
            env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set in .env");
        let rpc_http_url = env::var("RPC_HTTP_URL")
            .expect("RPC_HTTP_URL must be set in .env file (for getTransaction)");
        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "kafka:9092".to_string());
        let kafka_group_id =
            env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "req_producer".to_string());
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

        Self {
            pg_url,
            rpc_http_url,
            kafka_brokers,
            kafka_group_id,
            redis_url,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename_override(file_name).ok();
        Self::from_env()
    }
}

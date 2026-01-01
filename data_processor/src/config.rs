use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub pg_url: String,

    pub redis_url: String,

    pub clickhouse_url: String,
    pub clickhouse_user: String,
    pub clickhouse_password: String,
    pub clickhouse_database: String,

    pub kafka_brokers: String,
    pub kafka_group_id: String,

    pub new_accounts_limit: usize,
    pub new_accounts_key: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename_override(".env").ok();

        let pg_url =
            env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set in .env");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

        let clickhouse_url =
            env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://clickhouse:8123".to_string());
        let clickhouse_user =
            env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "clickhouse_user".to_string());
        let clickhouse_password =
            env::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| "clickhouse_password".to_string());
        let clickhouse_database =
            env::var("CLICKHOUSE_DATABASE").unwrap_or_else(|_| "events_db".to_string());

        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "kafka:9092".to_string());
        let kafka_group_id =
            env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "ch_consumer_group".to_string());

        let new_accounts_limit = env::var("NEW_ACCOUNTS_CACHE_LIMIT")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(10);
        let new_accounts_key =
            env::var("NEW_ACCOUNTS_KEY").unwrap_or_else(|_| "new_accounts".to_string());

        Self {
            pg_url,

            redis_url,

            kafka_brokers,
            kafka_group_id,

            clickhouse_url,
            clickhouse_user,
            clickhouse_password,
            clickhouse_database,

            new_accounts_limit,
            new_accounts_key,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename_override(file_name).ok();
        Self::from_env()
    }
}

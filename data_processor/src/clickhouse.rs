use crate::config::AppConfig;
use anyhow::Result;
use clickhouse::Client;
use log::info;

pub fn init_clickhouse_client(config: &AppConfig) -> Result<Client> {
    let clickhouse_url = config.clickhouse_url.clone();
    let client = Client::default()
        .with_url(&config.clickhouse_url)
        .with_user(&config.clickhouse_user)
        .with_password(&config.clickhouse_password)
        .with_database(&config.clickhouse_database);

    info!("ClickHouse client initialized for {clickhouse_url}");

    Ok(client)
}

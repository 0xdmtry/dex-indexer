use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename_override(".env").ok();

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

        Self { redis_url }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename_override(file_name).ok();
        Self::from_env()
    }
}

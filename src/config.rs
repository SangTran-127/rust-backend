use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub address: String,
}

#[derive(Deserialize)]
pub struct PostgresConfig {
    pub uri: String,
    pub max_conn: u32,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub web: WebConfig,
    pub postgres: PostgresConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<AppConfig, ConfigError> {
        Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("Cannot load env")
            .try_deserialize()
    }
}

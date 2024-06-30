use config::{Config, ConfigError, Environment};
use dotenv::var;
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
pub struct ProdConfig {
    pub web: WebConfig,
    pub postgres: PostgresConfig,
}

#[derive(Deserialize)]
pub struct DevConfig {
    pub dev_web: WebConfig,
    pub dev_postgres: PostgresConfig,
}

#[derive(Deserialize)]
pub struct DevEnv {
    pub app: DevConfig,
}

#[derive(Deserialize)]
pub struct ProdEnv {
    pub app: ProdConfig,
}

impl ProdConfig {
    pub fn from_env() -> Result<ProdConfig, ConfigError> {
        match var("ENV").as_deref() {
            Ok("prod") => {
                let config = Config::builder()
                    .add_source(Environment::default())
                    .build()
                    .expect("Cannot load env")
                    .try_deserialize::<ProdEnv>()?;
                Ok(ProdConfig {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }
            _ => {
                let config = Config::builder()
                    .add_source(Environment::default())
                    .build()
                    .expect("Cannot load env")
                    .try_deserialize::<DevEnv>()?;
                Ok(ProdConfig {
                    web: config.app.dev_web,
                    postgres: config.app.dev_postgres,
                })
            }
        }
    }
}

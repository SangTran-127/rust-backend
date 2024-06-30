use thiserror::Error;
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration Error")]
    Config(#[from] config::ConfigError),
}

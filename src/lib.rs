use error::AppError;

pub mod config;
pub mod db_connect;
pub mod error;

// mapping the Error for AppError
pub type AppResult<T> = Result<T, AppError>;

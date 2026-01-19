use thiserror::Error;
use kafka::error::Error as KafkaError;
use clickhouse::error::Error as CHError;
use redis::RedisError;
use std::io;
use r2d2::Error as ArcError;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Kafka error: {0}")]
    KafkaError(#[from] KafkaError),

    #[error("ClickHouse error: {0}")]
    ClickHouseError(#[from] CHError),

    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Serde error: {0}")]
    SerdeError(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Arc error: {0}")]
    ArcError(#[from] ArcError),

    #[error("Invalid data error: {0}")]
    InvalidDataError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("ClickHouse storage error: {0}")]
    ClickHouseStorageError(String),
    // 数据库相关错误
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl AppError {
    pub fn is_retryable(&self) -> bool {
        match self {
            AppError::KafkaError(_) | AppError::ClickHouseError(_) | AppError::RedisError(_) | AppError::IoError(_) | AppError::ProcessingError(_) => true,
            _ => false,
        }
    }

}

// 定义统一的返回结果
pub type AppResult<T> = Result<T, AppError>;
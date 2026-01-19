
use crate::config::{AppConfig};
use crate::utils::error::{AppError, AppResult};
use redis;
use redis::{TypedCommands };
use r2d2::{Pool,PooledConnection};
use std::sync::Arc;
pub struct RedisClient {
    pool: Arc<Pool<redis::Client>>,
}

impl RedisClient {
    pub fn new(cfg: &AppConfig) -> AppResult<Self> {
        let redis_url = cfg.storage.redis.url.clone();
        let client = redis::Client::open(redis_url.as_str()).map_err(|e| AppError::RedisError(e))?;
        let pool = Pool::builder()
                .max_size(10)
                .build(client)
                .map_err(|e | AppError::ArcError(e))?;
        Ok(Self { pool: Arc::new(pool) })
    }
    pub fn get_connection(&self) -> AppResult<PooledConnection<redis::Client>> {
        let conn: PooledConnection<redis::Client> = self.pool.get().map_err(|e| AppError::ArcError(e))?;
        Ok(conn)
    }
    pub fn ping(&self) -> AppResult<String> {
        let mut conn = self.get_connection()?;
        let pong: String = redis::cmd("PING").query(&mut conn).map_err(|e| AppError::RedisError(e))?;
        Ok(pong)
    }
    pub fn get_value(&self, key: &str) -> AppResult<Option<String>> {
        let mut conn = self.get_connection()?;
        let value: Option<String> = conn.get(key).map_err(|e| AppError::RedisError(e))?;
        Ok(value)
    }
    pub fn set_value(&self, key: &str, value: &str) -> AppResult<()> {
        let mut conn = self.get_connection()?;
        conn.set(key, value).map_err(|e| AppError::RedisError(e))?;
        Ok(())
    }
    pub fn delete_value(&self, key: &str) -> AppResult<()> {
        let mut conn = self.get_connection()?;
        conn.del(key).map_err(|e| AppError::RedisError(e))?;
        Ok(())
    }
    pub fn get_json_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<Option<T>> {
        let mut conn = self.get_connection()?;
        let json_str: Option<String> = conn.get(key).map_err(|e| AppError::RedisError(e))?;
        if let Some(json) = json_str {
            let value: T = serde_json::from_str(&json).map_err(|e| AppError::InvalidDataError(format!("Failed to deserialize JSON from Redis: {}", e)))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    pub fn set_json_value<T: serde::Serialize>(&self, key: &str, value: &T) -> AppResult<()> {
        let mut conn = self.get_connection()?;
        let json_str = serde_json::to_string(value).map_err(|e| AppError::InvalidDataError(format!("Failed to serialize value to JSON for Redis: {}", e)))?;
        conn.set(key, json_str).map_err(|e| AppError::RedisError(e))?;
        Ok(())
    }

}
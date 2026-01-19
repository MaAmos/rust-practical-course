use serde::{Deserialize, Serialize};
use std::sync::Arc;
use once_cell::sync::OnceCell;
use anyhow::{Result,Context};

// 定义全局的配置对象
static GLOBAL_APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app: AppSettings,
    pub kafka: KafkaSettings,
    pub storage: StorageSettings,
    pub processor: ProcessingSettings,

}


// 定义应用程序的基本信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub author_email: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KafkaSettings {
    pub brokers: Vec<String>,
    pub group_id: String,
    pub topic: String,
    pub max_retries: usize,
    pub retry_backoff_ms: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageSettings {
    pub clickhouse: ClickHouseSettings,
    pub redis: RedisSettings,
    pub sqlite: SqliteSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClickHouseSettings {
    pub url: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub connection_pool_size: u32,
    pub compression: String,
    pub connection_timeout_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisSettings {
    pub url: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SqliteSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingSettings {
    pub session_timeout_secs: u64,
    pub batch_interval_secs: u64,
    pub events_max_size: u64,
}


impl AppConfig {
    pub fn load() -> Result<Self>{
        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .context("Failed to build configuration")?;
        Ok(config.try_deserialize::<AppConfig>()?)
    }

    // 初始化全局配置
    pub fn init_global_config() -> Result<()> {
        let global_config = Self::load()?;
        let arc_config = Arc::new(global_config);
        GLOBAL_APP_CONFIG
            .set(arc_config)
            .map_err(|_| anyhow::anyhow!("Global config has already been initialized"))
    }

    // 获取全局配置的引用
    pub fn get_global_config() -> Result<Arc<AppConfig>> {
        GLOBAL_APP_CONFIG
            .get()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Global config is not initialized"))
    }
    pub fn get_kafka_config(&self) -> &KafkaSettings {
        &self.kafka
    }
}
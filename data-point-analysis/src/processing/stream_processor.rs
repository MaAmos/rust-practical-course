use crate::config::{AppConfig};
use crate::utils::error::{AppResult};
use std::time::{ Instant, Duration};
use crate::model::{DataPointLog};
use log::{info, error, debug, warn};
use super::session_splitting::Sessionizer;
use crate::storage::redis_client::RedisClient;
use crate::storage::clickhouse::ClickhouseClient;
use super::aggregator::RealTimeAggregator;
use std::sync::{Arc, RwLock};

// 实时数据流处理器
pub struct StreamProcessHandler<'a> {
    // Define fields and methods for processing messages
    stroage_payloads: Vec<String>,
    // 定义一个缓冲时间间隔 用于批量处理数据
    batch_interval_secs: u64,
    // 定义一个批量处理数据大小
    batch_max_size: u64,
    // 上一次处理数据的时间
    last_process_time: Instant,
    // 会话切割器实例
    sessionizer: Sessionizer<'a>,
    // 实时聚合器 (Shared)
    pub aggregator: Arc<RwLock<RealTimeAggregator>>,
}

impl<'a> StreamProcessHandler<'a> {
    pub fn new(cfg: &AppConfig, redis_client: RedisClient, clickhouse_client: &'a ClickhouseClient, aggregator: Arc<RwLock<RealTimeAggregator>>) -> Self {
        info!("Initializing StreamProcessHandler with batch_interval_secs: {} and batch_max_size: {}", cfg.processor.batch_interval_secs, cfg.processor.events_max_size);
        StreamProcessHandler {
            stroage_payloads: Vec::new(),
            batch_interval_secs: cfg.processor.batch_interval_secs,
            batch_max_size: cfg.processor.events_max_size,
            last_process_time: Instant::now(),
            sessionizer: Sessionizer::new(redis_client, cfg.processor.session_timeout_secs, clickhouse_client),
            aggregator,
        }
    }

    // 更新处理时间
    pub fn update_process_time(&mut self) {
        self.last_process_time = Instant::now();
    }
    // 清空数据
    pub fn clear_storage_payloads(&mut self) {
        self.stroage_payloads.clear();
    }
    // 更新当前待处理数据列表
    pub fn add_storage_payloads(&mut self, payload: String) {
        self.stroage_payloads.push(payload);
    }
    // 定义消费消息的处理方法
    pub async fn process_message(&mut self, payload: &str) {
        debug!("Processing message with length: {}", payload.len());
        // 判断一下 当前是否满足批量处理条件 超过条数 或者时间超过batch_interval_secs
        if self.stroage_payloads.len() as u64 >= self.batch_max_size || self.last_process_time.elapsed() >= Duration::from_secs(self.batch_interval_secs) {
            info!("Batch processing conditions met. Processing {} messages.", self.stroage_payloads.len());
            // 在这里进行会话数据的持久化存储动作
            self.sessionizer.save_sessions_to_clickhouse().await.unwrap_or_else(|e| {
                error!("Error saving sessions to ClickHouse: {:?}", e);
            });
            // 设置批量处理时间
            self.update_process_time();
            // 满足条件 则进行批量处理
            let playloads = self.stroage_payloads.clone();
            let _ = self.flush_data_batch(playloads).await;
            // 清空存储的payloads
            self.clear_storage_payloads();

        }
        self.add_storage_payloads(payload.to_string());
    }
    // 反序列化 埋点数据，并进行特征提取和存储
    pub async fn flush_data_batch(&mut self, playloads: Vec<String>) -> AppResult<()> {
        if self.stroage_payloads.is_empty() {
            return Ok(());
        }
        // 在这里实现批量处理逻辑 将缓冲区的数据进行存储
        for payload in playloads.iter() {
            // 把数据进行反序列化 拿到一个对象
            let event_message = match serde_json::from_str::<DataPointLog>(payload) {
                Ok(event_data) => {
                    let message = match  event_data.parse_inner_event() {
                        Ok(msg) => msg,
                        Err(e) => {
                            warn!("Error parsing inner event: {:?}", e);
                            continue;
                        }
                    };
                    message
                },
                Err(e) => {
                    warn!("Error deserializing message: {:?}", e);
                    continue;
                }
            };
            // 对每一条消息 进行特征提取、处理、存储
            // println!("Processed event message: {:?}", event_message);
            match self.sessionizer.split_message_to_session(&event_message).await {
                Ok(Some(session_info)) => {
                     // Feed into RealTimeAggregator
                     if let Ok(mut agg) = self.aggregator.write() {
                         agg.add_event(session_info);
                     } else {
                         error!("Failed to acquire write lock for aggregator");
                     }
                },
                Ok(None) => {}, // Skip
                Err(e) => {
                     warn!("Error splitting session: {:?}", e);
                }
            }
        }
        info!("Processing batch of {} messages", playloads.len());
        Ok(())
    }
}
pub mod utils;
pub mod config;
use config::AppConfig;
pub mod kafka;
use kafka::customer::run_consumer;

pub mod processing;
use processing::stream_processor::StreamProcessHandler;
use processing::aggregator::RealTimeAggregator;

pub mod model;
pub mod api;
use api::server::start_api_server;

pub mod storage;
use storage::redis_client::RedisClient;
use storage::clickhouse::ClickhouseClient;
use log::{info,error};
use env_logger;
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main (){
    print!("hello world");
    env_logger::init();
    if let Err(e) = AppConfig::init_global_config() {
        error!("Failed to initialize global config: {}", e);
    }
    match AppConfig::get_global_config(){
        Ok(cfg) => {
            // 初始化 redis 客户端
            let redis_client = match RedisClient::new(&cfg) {
                Ok(client) => client,
                Err(e) => {
                    error!("Failed to create Redis client: {}", e);
                    return;
                }
            };
            // 初始化 clickhouse 客户端
            let clickhouse_client = match ClickhouseClient::new(&cfg) {
                Ok(client) => {
                    //   创建 session_events 表
                    // match client.create_session_events_table().await {
                    //     Ok(_) => {
                    //         info!("Clickhouse table created successfully.");
                    //     },
                    //     Err(e) => {
                    //         error!("Failed to create Clickhouse table: {}", e);
                    //     }
                    // };
                    client
                },
                Err(e) => {
                    error!("Failed to create Clickhouse client: {}", e);
                    return;
                }
            };

            // Shared aggregator
            let aggregator = Arc::new(RwLock::new(RealTimeAggregator::new()));
            // Clone for API server
            let aggregator_for_server = aggregator.clone();

            // Start API Server in background task
            tokio::spawn(async move {
                start_api_server(aggregator_for_server, 3000).await;
            });

            // 新建一个流数据处理器
            let mut stream_processor = StreamProcessHandler::new(&cfg, redis_client, &clickhouse_client, aggregator);
            let _ = run_consumer(&cfg, &mut stream_processor).await;
        },
        Err(e) => {
            error!("Failed to get global config: {}", e);
        }
    };


}
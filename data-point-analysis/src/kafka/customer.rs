use std::thread;
use std::sync::Arc;
use std::time::Duration;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::{Error as KafkaError, KafkaCode};
use crate::utils::error::{AppError, AppResult};
use backon::{Retryable};
use crate::config::{AppConfig};
use crate::utils::retry_tools::retry_policy;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::utils::ctrl_c_tools::ctrl_c_handler;
use crate::processing::stream_processor::StreamProcessHandler;
// Kafka 消费者 消费 Topic逻辑
pub async fn run_consumer<'a>(cfg: &AppConfig, stream_processor: &mut StreamProcessHandler<'a>) -> AppResult<()> {
    let mut kafka_connector = create_consumer_connector(cfg).await?;
    println!("Starting Kafka consumer...");
    // 优雅退出标志（可在外部通过Ctrl+C手动触发）
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    // 监听Ctrl+C信号以优雅退出
    ctrl_c_handler(running_clone);
    // 定义一个缓冲区用于存储处理结果 方便进行批量处理数据
    while running.load(Ordering::SeqCst) {
         // 在这里拉取消息 进行消费 拉取最新的一条消息
        let mss = match kafka_connector.poll() {
            Ok(mss) => mss,
            Err(e) => {
                println!("Error polling Kafka: {:?}", e);
                thread::sleep(Duration::from_secs(60));
                continue;
            }
        };
        if mss.is_empty() {
            println!("No messages received");
            thread::sleep(Duration::from_secs(60));
            continue;
        }
        for msg in mss.iter() {
            for m in msg.messages() {
                let _ = m.offset;
                // println!("Received message at offset: {}", offset);
                let payload = match std::str::from_utf8(m.value) {
                    Ok(payload) => payload,
                    Err(_) => {
                        println!("Invalid message payload");
                        thread::sleep(std::time::Duration::from_secs(60));
                        continue;
                    }
                };
                stream_processor.process_message(payload).await;
            }
        }
        // 批次处理完成厚提交位点避免重复 消费
        if let Err(e) = kafka_connector.commit_consumed() {
            println!("Error committing offsets: {:?}", e);
            return Err(AppError::KafkaError(e));
        }

    }



    Ok(())

}


pub async fn create_consumer_connector(cfg: &AppConfig) -> AppResult<Consumer> {
    let kafka_config = cfg.get_kafka_config();
    let consumer = (|| async {
        Consumer::from_hosts(kafka_config.brokers.clone())
        .with_group(kafka_config.group_id.to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .with_topic(kafka_config.topic.to_owned())
        .create()
        .map_err(AppError::KafkaError)
    })
    .retry(retry_policy(kafka_config.max_retries, kafka_config.retry_backoff_ms))
    .await
    .map_err(|_| AppError::KafkaError(KafkaError::Kafka(KafkaCode::BrokerNotAvailable)))?;
    Ok(consumer)
}
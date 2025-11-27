use  tokio::time::{interval, Duration};
use crate::monitor::monitor_trait::Monitor;
use crate::monitor::types::{MonitorConfig, CheckResult};
// 1. 引入mpsc 通道概念 用于创建通道 并将接收端返回给调用者
use tokio::sync::mpsc;

use std::pin::Pin;

type MonitorResultReceiver = mpsc::Receiver<CheckResult>;
type PinnedReceiverFuture = Pin<Box<dyn Future<Output = MonitorResultReceiver> + Send>>;

pub struct AsyncMonitor {
}

impl AsyncMonitor {
    // 创建一个轮询的监控器
    pub fn create_interval_monitoring(target: Box<dyn Monitor>, config: MonitorConfig) -> PinnedReceiverFuture {
        Box::pin(async move {
            let (tx, rx) = mpsc::channel(100);
            tokio::spawn(async move {
                let mut interval: tokio::time::Interval = interval(Duration::from_secs(config.interval.unwrap_or(60)));
                loop {
                    interval.tick().await;
                    let check_result = target.check(&config).await;
                    if tx.send(check_result).await.is_err() {
                        println!("Error sending result to channel");
                    }

                }
            });
            // 返回接收端 对象
            rx

        })
    }
    // 创建一个单次监控
    pub fn create_once_monitoring( target: Box<dyn Monitor>, config: MonitorConfig) -> PinnedReceiverFuture {
        Box::pin(async move {
            let (tx, rx) = mpsc::channel(1);
            tokio::spawn(async move {
                let check_result = target.check(&config).await;
                if tx.send(check_result).await.is_err() {
                    println!("Error sending result to channel");
                }
            });
            // 返回接收端 对象
            rx
        })

    }

}
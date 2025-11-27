use  super::types::{MonitorConfig, CheckResult};
use crate::tools_types::MonitorType;
// 定义监控类型的 trait 后续的不同的监控类型都实现这个 trait
// 在这里添加 Send 和 Sync 作为父 trait
// Send: 允许在线程间移动
// Sync: 允许在线程间共享引用 (&T)
// 对于 tokio::spawn 和多线程异步，这两个通常都需要。
#[async_trait::async_trait]
pub trait Monitor: Send + Sync {
    async fn check(&self, config: &MonitorConfig) -> CheckResult;
    fn get_type(&self) -> MonitorType;
}
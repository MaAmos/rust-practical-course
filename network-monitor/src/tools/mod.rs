pub mod file_tool;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod http_tool;
pub use http_tool::*;

pub mod retry_tool;
// 获取当前的系统时间（毫秒）
pub fn unix_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

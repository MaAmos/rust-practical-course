// 增加重试机制函数
use backon::{ExponentialBuilder, Retryable};

pub fn default_retry_policy() -> ExponentialBuilder {
    ExponentialBuilder::default()
        .with_min_delay(std::time::Duration::from_secs(1))
        .with_max_delay(std::time::Duration::from_secs(5))
        .with_max_times(3)
}

pub fn critical_retry_policy() -> ExponentialBuilder {
    ExponentialBuilder::default()
        .with_min_delay(std::time::Duration::from_secs(1))
        .with_max_delay(std::time::Duration::from_secs(10))
        .with_max_times(5)
}

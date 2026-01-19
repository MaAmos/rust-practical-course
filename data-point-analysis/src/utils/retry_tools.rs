use backon::{ ExponentialBuilder };

// 定义重试策略
pub fn retry_policy(max_retries: usize, base_delay_ms: u64) -> ExponentialBuilder {
    ExponentialBuilder::default()
        .with_max_times(max_retries)
        .with_max_delay(std::time::Duration::from_millis(base_delay_ms))
}
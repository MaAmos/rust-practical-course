use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, MemoryMonitorResult};
use crate::tools_types::MonitorType;

pub struct MemoryMonitor {}

impl MemoryMonitor {
    pub fn new() -> Self {
        MemoryMonitor {}
    }
}

#[async_trait::async_trait]
impl Monitor for MemoryMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Memory,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Memory(MemoryMonitorResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Memory
    }
}
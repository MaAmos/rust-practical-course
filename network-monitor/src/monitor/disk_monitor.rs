use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, DiskMonitorResult};
use crate::tools_types::MonitorType;

pub struct DiskMonitor {}

impl DiskMonitor {
    pub fn new() -> Self {
        DiskMonitor {}
    }
}

#[async_trait::async_trait]
impl Monitor for DiskMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Disk,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Disk(DiskMonitorResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Disk
    }
}
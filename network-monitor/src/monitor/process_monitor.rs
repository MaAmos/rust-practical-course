use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail,ProcessMonitorResult};
use crate::tools_types::MonitorType;

pub struct ProcessMonitor {}

impl ProcessMonitor {
    pub fn new() -> Self {
        ProcessMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for ProcessMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Process,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Process(ProcessMonitorResult::default()),
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Process
    }
}
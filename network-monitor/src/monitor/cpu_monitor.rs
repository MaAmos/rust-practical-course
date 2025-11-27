use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, CpuMonitorResult};
use crate::tools_types::MonitorType;

pub struct CpuMonitor {

}

impl CpuMonitor {
    pub fn new() -> Self {
        CpuMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for CpuMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {

        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Cpu,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Cpu(CpuMonitorResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Cpu
    }
}
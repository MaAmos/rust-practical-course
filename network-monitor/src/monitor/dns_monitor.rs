use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, DnsMonitorResult};
use crate::tools_types::MonitorType;

pub struct DnsMonitor {}


impl DnsMonitor {
    pub fn new() -> Self {
        DnsMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for DnsMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Dns,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Dns(DnsMonitorResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Dns
    }
}
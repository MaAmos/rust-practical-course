use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, UdpMonitorResult};
use crate::tools_types::MonitorType;

pub struct UdpMonitor {}
impl UdpMonitor {
    pub fn new() -> Self {
        UdpMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for UdpMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Udp,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Udp(UdpMonitorResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Udp
    }
}
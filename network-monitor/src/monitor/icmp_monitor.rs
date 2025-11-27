
use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, IcmpMonitorResult};
use crate::tools_types::MonitorType;
// 定义http监控类型
pub struct IcmpMonitor {}

impl IcmpMonitor {
    pub fn new() -> Self {
        IcmpMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for IcmpMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Icmp,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Icmp(IcmpMonitorResult::default())
        }
    }
    fn get_type(&self) -> MonitorType {
        MonitorType::Icmp
    }

}
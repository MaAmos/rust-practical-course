use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, TracerouteMonitorResult};
use crate::tools_types::MonitorType;

pub struct TracerouteMonitor {}

impl TracerouteMonitor {
    pub fn new() -> Self {
        TracerouteMonitor {}
    }
}

#[async_trait::async_trait]
impl Monitor for TracerouteMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Traceroute,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Traceroute(TracerouteMonitorResult::default()),
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Traceroute
    }
}
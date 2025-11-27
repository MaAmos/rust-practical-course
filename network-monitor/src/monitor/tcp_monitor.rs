use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, TcpMonitorResult};
use crate::tools_types::MonitorType;

pub struct TcpMonitor {}

impl TcpMonitor {
    pub fn new() -> Self {
        TcpMonitor {}
    }
}

#[async_trait::async_trait]
impl Monitor for TcpMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Tcp,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Tcp(TcpMonitorResult::default()),
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Tcp
    }
}
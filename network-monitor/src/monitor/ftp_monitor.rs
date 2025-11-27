use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, FtpMonitorResult};
use crate::tools_types::MonitorType;
pub struct FtpMonitor {}

impl FtpMonitor {
    pub fn new() -> Self {
        FtpMonitor {}
    }
}


#[async_trait::async_trait]
impl Monitor for FtpMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Ftp,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Ftp(FtpMonitorResult::default()),
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Ftp
    }
}
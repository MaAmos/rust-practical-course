use super::monitor_trait::Monitor;
use super::types::{ MonitorConfig, CheckResult, CheckResultDetail, UnknownQueryResult};
use crate::tools_types::MonitorType;

pub struct UnknowMonitor {

}

impl UnknowMonitor {
    pub fn new() -> Self {
        UnknowMonitor {}
    }
}
#[async_trait::async_trait]
impl Monitor for UnknowMonitor {
    async fn check(&self, config: &MonitorConfig) -> CheckResult {

        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Unknown,
            target: config.target.clone(),
            status: true,
            details: CheckResultDetail::Unknown(UnknownQueryResult::default())
        }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Unknown
    }
}
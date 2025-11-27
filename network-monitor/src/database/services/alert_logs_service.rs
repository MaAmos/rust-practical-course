use crate::database::repositories::alert_logs_repo::AlertLogsRepository;
use crate::database::models::{AlertLogsModel, AlertLogsInsert};


pub struct AlertLogsService {
    // 以后有别的仓库再加：repo: AlertLogsRepository,
    repo: AlertLogsRepository
}

impl AlertLogsService {
    pub fn new( repo: AlertLogsRepository) -> Self {
        AlertLogsService { repo }
    }
    pub fn insert_alert_log(&self, alert_log: AlertLogsInsert) -> Result<usize, diesel::result::Error>{
        self.repo.insert_alert_logs(alert_log)
    }
    pub fn get_alert_logs_by_rules_id(&self, rule_id:i32) -> Result<Vec<AlertLogsModel>, diesel::result::Error>{
        self.repo.get_alert_logs_by_rules_id(rule_id)
    }
}
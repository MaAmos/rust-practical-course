use crate::database::repositories::alert_rules_repo::AlertRulesRepository;
use crate::database::models::{AlertRulesModel, AlertRulesInsert};

pub struct AlertRulesService {
    // 以后有别的仓库再加：repo: AlertRulesRepository,
    repo: AlertRulesRepository
}

impl AlertRulesService {
    pub fn new( repo: AlertRulesRepository) -> Self {
        AlertRulesService { repo }
    }
    pub fn insert_alert_rule(&self, alert_rule: AlertRulesInsert) -> Result<i64, diesel::result::Error>{
        self.repo.insert_alert_rule(alert_rule)
    }
    pub fn get_alert_rules_by_id(&self, rule_id:i32) -> Result<Vec<AlertRulesModel>, diesel::result::Error>{
        self.repo.get_alert_rules_by_id(rule_id)
    }
    pub fn get_alert_rules_by_enabled(&self, enabled_flag:bool, page: i64, page_size: i64)-> Result<(Vec<AlertRulesModel>, i64), diesel::result::Error> {
        self.repo.get_alert_rules_by_enabled( enabled_flag, page, page_size)
    }
}
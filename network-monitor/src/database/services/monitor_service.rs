// 业务逻辑层
use crate::database::repositories::monitor_repo::MonitorRepository;
use crate::database::models::{MonitorConfigModel, MonitorConfigUpdate, MonitorConfigInsert};
use std::sync::Arc;
#[derive(Clone)]
pub struct MonitorService {
    // 以后有别的仓库再加字段
    repo: Arc<MonitorRepository>,
}

impl MonitorService {
    pub fn new(repo: MonitorRepository) -> Self {
        MonitorService {
            repo: Arc::new(repo),

        }
    }
    pub fn get_all_monitors(&self, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        self.repo.get_all_monitors(limit_n, offset_n)
    }
    pub fn get_monitors_by_enabled(&self, enabled_flag:bool, page: i64, page_size: i64)-> Result<(Vec<MonitorConfigModel>, i64), diesel::result::Error> {
        self.repo.get_monitors_by_enabled(enabled_flag, page, page_size)
    }
    pub fn get_monitor_by_id(&self,id:i32) -> Result<Vec<MonitorConfigModel>, diesel::result::Error>{
        self.repo.get_monitor_by_id(id)
    }
    pub fn search_by_target(&self, target_pattern: &str, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        self.repo.search_by_target( target_pattern, limit_n, offset_n)
    }
    pub fn search_by_tag(&self, tag_str: &str, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        self.repo.search_by_tag(tag_str, limit_n, offset_n)
    }
    pub fn update_monitor_by_id(&self,id:i32, monitor: MonitorConfigUpdate) -> Result<usize, diesel::result::Error>{
        self.repo.update_monitor_by_id(id, monitor)
    }
    pub fn insert_monitor(&self, new_monitor: MonitorConfigInsert) -> Result<i64, diesel::result::Error>{
        self.repo.insert_monitor(new_monitor)
    }
    pub fn delete_monitor(&self, id:i32) -> Result<usize, diesel::result::Error>{
        self.repo.delete_monitor(id)
    }

}
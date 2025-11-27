
use crate::database::repositories::check_result_repo::CheckResultRepository;
use crate::database::models::{CheckResultModel, CheckResultModelInsert};
use std::sync::Arc;
#[derive(Clone)]
pub struct CheckResultService {
    // 以后有别的仓库再加：repo: CheckResultRepository,
    repo: Arc<CheckResultRepository>
}
impl CheckResultService {
    pub fn new( repo: CheckResultRepository) -> Self {
        CheckResultService { repo: Arc::new(repo) }
    }
    pub fn insert_check_result(&self, check_result: CheckResultModelInsert) -> Result<usize, diesel::result::Error>{
        self.repo.insert_check_result(check_result)
    }
    pub fn get_check_result_by_monitor_id(&self, monitor_id:i32, page: i64, page_size: i64) -> Result<(Vec<CheckResultModel>, i64), diesel::result::Error>{
        self.repo.get_check_result_by_monitor_id(monitor_id, page, page_size)
    }

}
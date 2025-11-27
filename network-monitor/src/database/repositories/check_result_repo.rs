use diesel::prelude::*;
use crate::database::connect_db::{SqlitePool, get_connection};
use crate::database::models::{CheckResultModel, CheckResultModelInsert};
use crate::database::schema::{check_result};
use std::sync::Arc;
#[derive(Clone)]
pub struct CheckResultRepository {
    pool: Arc<SqlitePool>,
}

impl CheckResultRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        CheckResultRepository { pool }
    }
    // 在这里添加与检查结果相关的数据库操作方法
    pub fn insert_check_result(&self, check_result: CheckResultModelInsert) -> Result<usize, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        diesel::insert_into(check_result::table)
            .values(&check_result)
            .execute(&mut conn)
    }
    // 按监控项ID分页
    pub fn get_check_result_by_monitor_id(&self, monitor_id:i32, page: i64, page_size: i64)-> Result<(Vec<CheckResultModel>, i64), diesel::result::Error> {
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        let page_no = page.max(1);
        let page_sz = page_size.max(1);
        let offset = (page_no -1) * page_sz;
        let total:i64 = check_result::table
            .filter(check_result::monitor_id.eq(monitor_id))
            .count()
            .get_result(&mut conn)?;
        // 执行数据库操作
         let results =  check_result::table
                .filter(check_result::monitor_id.eq(monitor_id))
                .order(check_result::id.desc())
                .limit(page_sz)
                .offset(offset)
                .load::<CheckResultModel>(&mut conn)?;
        Ok((results, total))
    }
    // 删除监控结果信息
    pub fn delete_check_result_by_monitor_id(&self, monitor_id: i32) -> Result<usize,diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        diesel::delete(check_result::table.filter(check_result::monitor_id.eq(monitor_id)))
            .execute(&mut conn)
    }
    // 删除监控结果信息 -- 根据id
    pub fn delete_check_result_by_id(&self,id:i32) -> Result<usize, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        diesel::delete(check_result::table.filter(check_result::id.eq(id)))
            .execute(&mut conn)
    }

}
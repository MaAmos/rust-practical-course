
use diesel::prelude::*;
use crate::database::connect_db::{SqlitePool, get_connection};
use crate::database::models::{MonitorConfigModel, MonitorConfigUpdate, MonitorConfigInsert};
use crate::database::schema::{monitor_config};
use diesel::sql_types::BigInt;
use std::sync::Arc;
define_sql_function! {
    /// 获取最后插入的行 ID
    fn last_insert_rowid() -> BigInt;
}

pub struct MonitorRepository {
    pool: Arc<SqlitePool>,
}

impl MonitorRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        MonitorRepository { pool }
    }
    // 在这里添加与监控数据相关的数据库操作方法
    // 获取全部的监控数据 （已有索引时最好加分页，避免大表全扫描）
    pub fn get_all_monitors(&self, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        // 执行数据库操作
        monitor_config::table
            .order(monitor_config::id.desc())
            .limit(limit_n)
            .offset(offset_n)
            .load::<MonitorConfigModel>(&mut conn)
    }
    // 按启用状态分页（利用 idx_monitor_config_enabled_id   索引）
    pub fn get_monitors_by_enabled(&self, enabled_flag:bool, page: i64, page_size: i64)-> Result<(Vec<MonitorConfigModel>, i64), diesel::result::Error> {
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        let page_no = page.max(1);
        let page_sz = page_size.max(1);
        let offset = (page_no -1) * page_sz;
        let enabled_v = if enabled_flag {1} else {0};

        let total:i64 = monitor_config::table
            .filter(monitor_config::enabled.eq(enabled_v))
            .count()
            .get_result(&mut conn)?;
        // 执行数据库操作
       let results =  monitor_config::table
            .filter(monitor_config::enabled.eq(enabled_v))
            .order(monitor_config::id.desc())
            .limit(page_sz)
            .offset(offset)
            .load::<MonitorConfigModel>(&mut conn)?;
        Ok((results, total))
    }


    // 根据ID获取相关的监控配置
    pub fn get_monitor_by_id(&self,id:i32) -> Result<Vec<MonitorConfigModel>, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        monitor_config::table.find(id).first(&mut conn).map(|monitor:MonitorConfigModel| vec![monitor])
    }
    // 目标精确或前缀搜索
    pub fn search_by_target(&self, target_pattern: &str, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        let pattern = format!("{}%", target_pattern); // 前缀匹配
        monitor_config::table
            .filter(monitor_config::target.like(pattern))
            .order(monitor_config::id.desc())
            .limit(limit_n)
            .offset(offset_n)
            .load::<MonitorConfigModel>(&mut conn)
    }
    // 标签过滤 利用tag索引
    pub fn search_by_tag(&self, tag_str: &str, limit_n: i64, offset_n: i64)-> Result<Vec<MonitorConfigModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        monitor_config::table
            .filter(monitor_config::tag.eq(tag_str))
            .order(monitor_config::id.desc())
            .limit(limit_n)
            .offset(offset_n)
            .load::<MonitorConfigModel>(&mut conn)
    }
    // 根据ID 来修改监控配置项
    pub fn update_monitor_by_id(&self,id:i32, monitor: MonitorConfigUpdate) -> Result<usize, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        diesel::update(monitor_config::table.find(id))
            .set(monitor)
            .execute(&mut conn)
    }
    // 新增监控项
    pub fn insert_monitor(&self, monitor: MonitorConfigInsert) -> Result<i64, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        diesel::insert_into(monitor_config::table)
            .values(&monitor)
            .execute(&mut conn)?;
        diesel::select(last_insert_rowid()).get_result::<i64>(&mut conn).map(|id: i64| id )
    }
    // 删除监控项
    pub fn delete_monitor(&self, id:i32) -> Result<usize, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        diesel::delete(monitor_config::table.find(id))
            .execute(&mut conn)
    }

}
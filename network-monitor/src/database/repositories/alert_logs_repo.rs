use diesel::prelude::*;
use crate::database::connect_db::{SqlitePool, get_connection};
use crate::database::models::{AlertLogsModel, AlertLogsInsert};
use crate::database::schema::{alert_logs};


pub struct AlertLogsRepository {
    pool: SqlitePool,
}

impl AlertLogsRepository{
    pub fn new(pool: SqlitePool) -> Self {
        AlertLogsRepository { pool }
    }
    // 获取全部的提醒日志
    pub fn get_all_alert_logs(&self,limit_n: i64, offset: i64) -> Result<Vec<AlertLogsModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        alert_logs::table
            .order(alert_logs::created_at.desc())
            .limit(limit_n)
            .offset(offset)
            .load::<AlertLogsModel>(&mut conn)
    }
    // 删除日志
    pub fn delete_alert_logs_by_id(&self, id: i32) -> Result<usize, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        diesel::delete(alert_logs::table.filter(alert_logs::id.eq(id)))
            .execute(&mut conn)
    }
    // 根据 rules_id查询日志
    pub fn get_alert_logs_by_rules_id(&self, rules_id: i32) -> Result<Vec<AlertLogsModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        alert_logs::table
            .filter(alert_logs::alert_rules_id.eq(rules_id))
            .order(alert_logs::created_at.desc())
            .load::<AlertLogsModel>(&mut conn)
    }
    // 根据 monitor_id查询日志
    pub fn get_alert_logs_by_monitor_id(&self, monitor_id: i32) -> Result<Vec<AlertLogsModel>,diesel::result::Error>{
        let mut conn = get_connection(&self.pool);
        alert_logs::table
            .filter(alert_logs::monitor_id.eq(monitor_id))
            .order(alert_logs::created_at.desc())
            .load::<AlertLogsModel>(&mut conn)

    }
    // 插入一条日志
    pub fn insert_alert_logs(&self, alert_logs_insert:AlertLogsInsert )-> Result<usize, diesel::result::Error>{
        let mut conn = get_connection(&self.pool);
        diesel::insert_into(alert_logs::table)
            .values(&alert_logs_insert)
            .execute(&mut conn)
    }



}
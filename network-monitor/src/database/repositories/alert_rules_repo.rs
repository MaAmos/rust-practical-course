use diesel::prelude::*;
use crate::database::connect_db::{SqlitePool, get_connection};
use crate::database::models::{AlertRulesModel, AlertRulesInsert, AlertRulesUpdate};
use crate::database::schema::{alert_rules};
use diesel::sql_types::BigInt;
define_sql_function! {
    /// 获取最后插入的行 ID
    fn last_insert_rowid() -> BigInt;
}
pub struct AlertRulesRepository{
    pool: SqlitePool,
}

impl AlertRulesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        AlertRulesRepository { pool }
    }
    // 在这里添加与告警规则相关的数据库操作方法
    pub fn get_all_alert_rules(&self, limit_n: i64, offset_n: i64)-> Result<Vec<AlertRulesModel>, diesel::result::Error> {
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        // 执行数据库操作
        alert_rules::table
            .order(alert_rules::id.desc())
            .limit(limit_n)
            .offset(offset_n)
            .load::<AlertRulesModel>(&mut conn)
    }
    // 其他与告警规则相关的方法可以在这里添加
    pub fn get_alert_rules_by_enabled(&self, enabled_flag:bool, page: i64, page_size: i64)-> Result<(Vec<AlertRulesModel>, i64), diesel::result::Error> {
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        let page_no = page.max(1);
        let page_sz = page_size.max(1);
        let offset = (page_no -1) * page_sz;
        let enabled_v = if enabled_flag {1} else {0};

        let total:i64 = alert_rules::table
            .filter(alert_rules::enabled.eq(enabled_v))
            .count()
            .get_result(&mut conn)?;
        // 执行数据库操作
         let results =  alert_rules::table
                .filter(alert_rules::enabled.eq(enabled_v))
                .order(alert_rules::id.desc())
                .limit(page_sz)
                .offset(offset)
                .load::<AlertRulesModel>(&mut conn)?;
        Ok((results, total))
    }
    // 根据ID获取相关的告警规则
    pub fn get_alert_rules_by_id(&self,id:i32) -> Result<Vec<AlertRulesModel>, diesel::result::Error>{
        // 获取当前对数据库的链接
        let mut conn = get_connection(&self.pool);
        alert_rules::table.find(id).first(&mut conn).map(|rule:AlertRulesModel| vec![rule])
    }
    // 插入一条规则
    pub fn insert_alert_rule(&self, new_rule: AlertRulesInsert) -> Result<i64, diesel::result::Error>{
        let mut conn = get_connection(&self.pool);
        diesel::insert_into(alert_rules::table)
            .values(&new_rule)
            .execute(&mut conn)?;
        diesel::select(last_insert_rowid()).get_result(&mut conn).map(|id: i64| id )
    }
    // 更新一条规则
    pub fn update_alert_rule(&self, rule_id: i32, updated_rule: AlertRulesUpdate) -> Result<usize, diesel::result::Error>{
        let mut conn = get_connection(&self.pool);
        diesel::update(alert_rules::table.filter(alert_rules::id.eq(rule_id)))
            .set(&updated_rule)
            .execute(&mut conn)
    }

}
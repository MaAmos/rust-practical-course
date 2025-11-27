use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::database::schema::{monitor_config, check_result, alert_rules, alert_logs, notification_channels};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = monitor_config)]
pub struct MonitorConfigModel {
    pub id: i32,
    pub name: Option<String>,
    pub target: String,
    pub method: Option<String>,
    pub monitor_type: String,
    pub interval_ms: Option<i32>,
    pub timeout_ms: i32,
    pub config_json: Option<String>,
    pub enabled: i32,
    pub tag: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// 更新用结构（不含 id / created_at / updated_at）
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = monitor_config)]
pub struct MonitorConfigUpdate {
    pub name: Option<String>,
    pub target: Option<String>,
    pub method: Option<String>,
    pub monitor_type: Option<String>,
    pub interval_ms: Option<i32>,
    pub timeout_ms: Option<i32>,
    pub config_json: Option<String>,
    pub enabled: Option<i32>,
    pub tag: Option<String>,
}
// 新增用结构体 （不包含ID ，updated_at） ）
#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = monitor_config)]
pub struct MonitorConfigInsert {
    pub name: Option<String>,
    pub target: String,
    pub method: Option<String>,
    pub monitor_type: String,
    pub interval_ms: Option<i32>,
    pub timeout_ms: i32,
    pub config_json: Option<String>,
    pub enabled: i32,
    pub tag: Option<String>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = check_result)]
pub struct CheckResultModel {
    pub id: i32,
    pub monitor_id: i32,
    pub monitor_type: String,
    pub status: i32,
    pub response_time: i32,
    pub metadata_json: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


// 插入监控结果数据
#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = check_result)]
pub struct CheckResultModelInsert {
    pub monitor_id: i32,
    pub monitor_type: String,
    pub status: i32,
    pub response_time: i32,
    pub metadata_json: Option<String>,
}





#[derive(Queryable, Debug, Clone, Serialize, Deserialize, Selectable)]
#[diesel(table_name = alert_rules)]
pub struct AlertRulesModel {
    pub id: i32,
    pub monitor_id: i32,
    pub alert_type: String,
    pub config_json: Option<String>,
    pub enabled: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = alert_rules)]
pub struct AlertRulesInsert {
    pub monitor_id: i32,
    pub alert_type: String,
    pub config_json: Option<String>,
}

#[derive(AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = alert_rules)]
pub struct AlertRulesUpdate {
    pub id: i32,
    pub monitor_id: i32,
    pub alert_type: String,
    pub config_json: Option<String>,
}



#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = alert_logs)]
pub struct AlertLogsModel {
    pub id: i32,
    pub alert_rules_id: i32,
    pub monitor_id: i32,
    pub alert_type: String,
    pub status: i32,
    pub message: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = alert_logs)]
pub struct AlertLogsInsert {
    pub alert_rules_id: i32,
    pub monitor_id: i32,
    pub alert_type: String,
    pub status: i32,
    pub message: Option<String>,
}



#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = notification_channels)]
pub struct NotificationChannelsModel {
    pub id: i32,
    pub name: Option<String>,
    pub channel_type: String,
    pub config_json: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = notification_channels)]
pub struct NotificationChannelsUpdate {
    pub name: Option<String>,
    pub channel_type: String,
    pub config_json: Option<String>,
}
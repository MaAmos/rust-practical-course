pub mod monitor_handlers;
pub mod check_result_handlers;
pub mod alert_rule_handlers;
pub mod alert_log_handlers;
use serde::{ Deserialize, Serialize};
use actix_web::{HttpResponse};
// 定义一个返回统一的数据结构，方便前端进行数据处理
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponseObj<T> {
    code: usize,
    message: String,
    data: T,
}

pub async fn default_handler() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: "Network Monitor API is running".to_string(),
    }))
}


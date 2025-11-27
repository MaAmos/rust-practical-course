
use actix_web::{web, HttpResponse, Result};
use super::DefaultResponseObj;
use crate::database::services::AlertLogsService;



pub async  fn get_all_alert_logs(
    alert_logs_service: web::Data<AlertLogsService>,
    query: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let alert_logs = alert_logs_service.get_alert_logs_by_rules_id(*query)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get alert logs: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: alert_logs,
    }))
}
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use crate::database::models::{AlertRulesInsert};
use super::DefaultResponseObj;
use crate::database::services::AlertRulesService;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
}

pub async  fn get_all_alert_rules(
    alert_rules_service: web::Data<AlertRulesService>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let no = query.page_no.unwrap_or(1);
    let size = query.page_size.unwrap_or(20);
    let alert_rules = alert_rules_service.get_alert_rules_by_enabled(true, no, size)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get alert rules: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: alert_rules,
    }))
}

pub async  fn insert_alert_rule(
    alert_rules_service: web::Data<AlertRulesService>,
    new_alert_rule: web::Json<AlertRulesInsert>,
) -> Result<HttpResponse, actix_web::Error> {
    let insert_result = alert_rules_service.insert_alert_rule(new_alert_rule.into_inner())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to insert alert rule: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: insert_result,
    }))
}
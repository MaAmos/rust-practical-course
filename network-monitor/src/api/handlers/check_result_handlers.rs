use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use super::DefaultResponseObj;
use crate::database::services::CheckResultService;
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
}
pub async fn get_check_results_by_monitor_id(
    monitor_id: web::Path<i32>,
    check_result_service: web::Data<CheckResultService>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let no = query.page_no.unwrap_or(1);
    let size = query.page_size.unwrap_or(2000);
    let check_results = check_result_service.get_check_result_by_monitor_id(monitor_id.into_inner(), no, size)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get check results: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: check_results.0,
    }))
}
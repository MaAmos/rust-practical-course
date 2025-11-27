use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use crate::database::services::MonitorService;
use crate::database::models::{MonitorConfigInsert, MonitorConfigUpdate};
use super::DefaultResponseObj;
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page_no: Option<i64>,
    pub page_size: Option<i64>,
}

pub async  fn get_all_monitors(
    monitor_service: web::Data<MonitorService>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let no = query.page_no.unwrap_or(1);
    let size = query.page_size.unwrap_or(20);
    let monitors  = monitor_service.get_monitors_by_enabled(true,no,size)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get monitors: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: monitors.0,
    }))
}


pub async  fn insert_monitors(
    monitor_service: web::Data<MonitorService>,
    new_monitor: web::Json<MonitorConfigInsert>,
) -> Result<HttpResponse, actix_web::Error>  {
    let insert_result = monitor_service.insert_monitor(new_monitor.into_inner())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to insert monitor: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: insert_result,
    }))
}

pub async fn get_monitors_by_id(
   monitor_service: web::Data<MonitorService>,
    query: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let monitor = monitor_service.get_monitor_by_id(query.into_inner())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to get monitor: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: monitor,
    }))

}

pub async  fn update_monitors_by_id(
    monitor_service: web::Data<MonitorService>,
    query: web::Path<i32>,
    new_monitor: web::Json<MonitorConfigUpdate>,
) -> Result<HttpResponse, actix_web::Error> {
    let _update_count = monitor_service.update_monitor_by_id(query.into_inner(), new_monitor.into_inner())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to update monitor: {}", e))
        })?;
    Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: _update_count,
    }))
}

pub async fn delete_monitors_by_id(
    monitor_service: web::Data<MonitorService>,
    query: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let delete_count = monitor_service.delete_monitor(query.into_inner())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to delete monitor: {}", e))
        })?;
     Ok(HttpResponse::Ok().json(DefaultResponseObj {
        code: 200,
        message: "OK".to_string(),
        data: delete_count,
    }))
}
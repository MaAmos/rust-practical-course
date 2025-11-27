use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    print!("配置 API 路由...");
    cfg.service(
        web::scope("/api")
            .route("/monitors", web::get().to(crate::api::handlers::monitor_handlers::get_all_monitors))
            .route("/monitors", web::post().to(crate::api::handlers::monitor_handlers::insert_monitors))
            .route("/monitors/{id}", web::get().to(crate::api::handlers::monitor_handlers::get_monitors_by_id))
            .route("/monitors/{id}", web::put().to(crate::api::handlers::monitor_handlers::update_monitors_by_id))
            .route("/monitors/{id}", web::delete().to(crate::api::handlers::monitor_handlers::delete_monitors_by_id))
            .route("/monitors/{id}/results", web::get().to(crate::api::handlers::check_result_handlers::get_check_results_by_monitor_id))
            .route("/alert_rules", web::get().to(crate::api::handlers::alert_rule_handlers::get_all_alert_rules))
            .route("/alert_rules", web::post().to(crate::api::handlers::alert_rule_handlers::insert_alert_rule))
            .route("/alert_logs", web::get().to(crate::api::handlers::alert_log_handlers::get_all_alert_logs))
            .route("/", web::get().to(crate::api::handlers::default_handler))
    );

}

// 错误处理中间件

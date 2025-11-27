pub mod tools_types;// 定义公用的枚举值模块 这样在其他的地方 可以直接引用
use tools_types::{ convert_to_monitor_config, MonitorType };

mod async_monitor;
use async_monitor::AsyncMonitor;


// 定义文件读取操作
mod tools;
// use tools::file_tool::{read_json_file};


pub mod monitor;
use monitor::{ MonitorFactory } ;

use crate::tools_types::SelfDefineMonitorConfig;

// 定义数据库相关模块
pub mod database;
use database::{ connect_db, repositories, services };
use database::models::{MonitorConfigModel, CheckResultModelInsert};
// 定义API模块
pub mod api;

// 定义告警规则模块
pub mod alerts;
use alerts::AlertsEngine;

use std::sync::Arc;

// 创建 HTTP服务
use actix_web::{web, App, HttpServer};

// 重试机制
use crate::tools::retry_tool::{default_retry_policy, critical_retry_policy};
use backon::Retryable;

async fn schedule_monitoring_task(monitor_website_list: Vec<MonitorConfigModel>, check_result_service: services::check_result_service::CheckResultService) {

    // 存储线程的运行结果
    let mut result_monitors = Vec::new();
    for monitor in monitor_website_list {
        let monitor_config_json = (|| async {
            serde_json::from_str::<SelfDefineMonitorConfig>(&monitor.config_json.clone().unwrap_or_default())
                .map_err(|e| {
                    e
                })
        })
        .retry(default_retry_policy())
        .await
        .unwrap_or_else(|_| SelfDefineMonitorConfig { target: None, monitor_type: MonitorType::Unknown, method: None, interval: None, timeout: None, params: None, headers: None, body: None, content_evaluation_rules: None, alert_rules: None });
        // 创建一个具体的监控器实例
        let target = MonitorFactory::create_monitor(monitor_config_json.monitor_type);
        // 把监控配置文件中的对象转换成监控器检测时需要的对象
        let monitor_config = convert_to_monitor_config(&monitor);
        // 根据不同的监控类型参数 创建不同的告警引擎
        let monitor_alert_engine = AlertsEngine::new(monitor_config_json.alert_rules.clone().unwrap_or_default());
        // 如果是一个 轮询监控 创建一个轮询监控器
        let async_monitor = match monitor_config_json.interval {
                Some(_) => {
                    // 创建一个轮询监控调度器
                     AsyncMonitor::create_interval_monitoring(target, monitor_config).await
                },
                None => {
                    // 创建一个单次监控调度器
                    AsyncMonitor::create_once_monitoring(target, monitor_config).await
                }
            };
        result_monitors.push((monitor.id,monitor_alert_engine, async_monitor));
    }
    // 处理所有的监控结果
    for (monitor_id, monitor_alert_engine,  monitor_receiver) in result_monitors {
        // 使用克隆的服务层结构体，避免后续会被循环引用 无法释放
        let check_result_service_move =check_result_service.clone();
        // 使用tokio::spawn异步任务来监控不同类型的调度器，避免阻塞主进程
        tokio::spawn(async move {
            // 这里使用一个循环来接收所有的监控结果
            let mut result_receiver = monitor_receiver ;
            while let Some(result) = result_receiver.recv().await {
                println!("Received result: {:?}", result.target);
                // 在这里 记录 监控规则的数据
                // 增加重试机制
                let _ = (|| async {
                    check_result_service_move.insert_check_result(CheckResultModelInsert {
                        monitor_id: monitor_id,
                        monitor_type: result.monitor_type.to_string(),
                        status: if result.status { 1 } else { 0 },
                        response_time: 0,
                        metadata_json: Some(serde_json::to_string(&result.details).unwrap_or_default()),
                    })
                })
                .retry(default_retry_policy())
                .when(|e|true)
                .await
                .map_err(|e| {
                    eprintln!("Failed to insert check result after retries: {}", e);
                });
                // 调用告警引擎 进行告警检查
                let _ = monitor_alert_engine.check(result).await;
            }
        });

    }
    println!("所有监控已启动，程序将继续运行");
    // // 你可以根据需要调整这个时间或者使用其他方式保持程序运行
    // tokio::time::sleep(Duration::from_secs(60)).await; // 让程序运行60s然后程序就会退出


}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("网络监控器 启动...");
    // 初始化数据库
    // let pool = Arc::new(connect_db::establish_connection());
    let pool = match connect_db::establish_database_connection().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to establish database connection: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };
    // 初始化 监控配置表
    let monitor_repo: repositories::monitor_repo::MonitorRepository = repositories::monitor_repo::MonitorRepository::new(Arc::clone(&pool));
    let monitor_service = services::monitor_service::MonitorService::new(monitor_repo);

    // 初始化 监控结果
    let check_result_repo = repositories::check_result_repo::CheckResultRepository::new(Arc::clone(&pool));
    let check_result_service = services::check_result_service::CheckResultService::new(check_result_repo);

    // 读取监控配置表的数据
    let monitor_website_list = (|| async { monitor_service.get_monitors_by_enabled(true, 1, 1000) })
    .retry(&default_retry_policy())
    .when(|e| {
        println!("读取监控配置表数据失败: {}", e);
        true
    }).await
    .map_err(|e| {
        eprintln!("Failed to read monitor list after retries: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Read monitor list failed")
    })?;
    // let monitor_website_list = read_json_file("monitor_list.json");
    let check_result_service_clone = check_result_service.clone();
    tokio::spawn(async move {
        let _ = schedule_monitoring_task(monitor_website_list.0, check_result_service_clone).await;
    });

    // 启动 HTTP服务
    print!("启动 HTTP服务 在0.0.0.0:8080...");
    //
    HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new(monitor_service.clone()))
            .app_data(web::Data::new(check_result_service.clone()))
            .configure(api::routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

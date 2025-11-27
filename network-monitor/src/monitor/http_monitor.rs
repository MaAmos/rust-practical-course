
use super::monitor_trait::Monitor;
use super::types::{MonitorConfigDetail, UnknownQueryResult,MonitorConfig, CheckResult, HttpMonitorConfig, CheckResultDetail };
use reqwest::Client;
use std::collections::HashMap;
use crate::tools_types::{MonitorType,HttpMethodTypes, HttpBody,HttpMonitorResult,StatusCategory,BasicAvailability, SecurityHeaders, PerformanceTimings,ContentVerificationResult,AdvancedAvailability, ContentVerificationRulesSingle, ContentVerificationRules, ContentVerificationRulesResult, StatusInfo};
use crate::tools::{ unix_now_ms, get_dns_tcp_tls_performance};
use uuid::Uuid;
use regex::Regex;
// 定义http监控类型
pub struct HttpMonitor {
    client: Client,
}

impl HttpMonitor {
    pub fn new() -> Self {
        HttpMonitor {
            client: Client::builder()
                .redirect(reqwest::redirect::Policy::limited(5)) // 重定向次数限制为5次
                .build().expect("Failed to create HTTP client"),
        }
    }
    // 获取一个http 客户端链接 根据 method 方法来判断 创建一个http 链接
    pub fn get_client(&self, config: &HttpMonitorConfig) -> reqwest::RequestBuilder {
        //   创建一个http 请求
        let mut  request_client = match config.method {
            HttpMethodTypes::Get => self.client.get(config.url.as_str()),
            HttpMethodTypes::Post => self.client.post(config.url.as_str()),
            HttpMethodTypes::Put => self.client.put(config.url.as_str()),
            HttpMethodTypes::Delete => self.client.delete(config.url.as_str()),
            HttpMethodTypes::Head => self.client.head(config.url.as_str()),
            HttpMethodTypes::Patch => self.client.patch(config.url.as_str()),
            _ => self.client.get(config.url.as_str()), // 默认使用 GET 方法
        };

        // 设置超时时间
       request_client = request_client.timeout(std::time::Duration::from_millis(config.timeout));
        if let Some(headers) = &config.headers {
            request_client = request_client.headers(headers.clone());
        }

       if let Some(body) = &config.body {
           match body {
               HttpBody::Text(text) => {
                   request_client = request_client.body(text.clone());
               },
               HttpBody::Binary(bin) => {
                   request_client = request_client.body(bin.clone());
               },
               HttpBody::Json(json_value) => {
                   request_client = request_client.json(json_value);
               },
               HttpBody::Empty => {request_client = request_client.body("");},
           }
       }

       request_client
    }
    // 解析成基础的响应结果结构体
    fn create_basic_result(&self, status_code: u16, version: reqwest::Version, headers: &reqwest::header::HeaderMap, content_length: Option<u64> ) -> BasicAvailability {
        BasicAvailability {
            is_reachable: true, // 代表这个target 是可以访问的
            dns_resolvable: true,  // 代表这个target 的域名是可以解析的
            tcp_connect_success: true, // 代表这个target 的 TCP 连接是成功的
            res_received: true,
            res_status_code: Some(status_code),
            res_status_category: StatusCategory::from_status_code(status_code),
            protocol_version: format!("{:?}", version), // 获取HTTP协议版本
            res_content_type: headers.get(reqwest::header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).map(|s| s.to_string()),
            res_content_length: content_length.unwrap_or(0),
            res_charset: headers.get(reqwest::header::ACCEPT_CHARSET).and_then(|v| v.to_str().ok()).map(|s| s.to_string()),
        }
    }
    // 解析相关头信息
    fn create_headers_result(&self, headers: &reqwest::header::HeaderMap) -> SecurityHeaders {
        SecurityHeaders {
            strict_transport_security: headers.get("Strict-Transport-Security").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            content_security_policy: headers.get("Content-Security-Policy").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            x_content_type_options: headers.get("X-Content-Type-Options").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            x_frame_options: headers.get("X-Frame-Options").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            x_xss_protection: headers.get("X-XSS-Protection").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            referrer_policy: headers.get("Referrer-Policy").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            feature_policy: headers.get("Feature-Policy").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            permissions_policy: headers.get("Permissions-Policy").as_ref().map(|v| v.to_str().unwrap_or("").to_string()),
            security_headers_ok: self.check_security_headers_ok(&headers),
        }
    }
    fn check_security_headers_ok(&self, headers: &reqwest::header::HeaderMap) -> bool {
        let mut score = 0;
        if headers.get("Strict-Transport-Security").is_some() {
            score += 1;
        }
        if headers.get("Content-Security-Policy").is_some() {
            score += 1;
        }
        if headers.get("X-Content-Type-Options").is_some() {
            score += 1;
        }
        if headers.get("X-Frame-Options").is_some() {
            score += 1;
        }
        if headers.get("X-XSS-Protection").is_some() {
            score += 1;
        }
        if headers.get("Referrer-Policy").is_some() {
            score += 1;
        }
        if headers.get("Permissions-Policy").is_some() {
            score += 1;
        }
        score >= 4 // 例如，至少有4个安全头部存在则认为安全头部检查通过
    }

    // 性能参数
    fn create_performance_timings(&self, start_time: std::time::Instant, start_time_unix: u128) -> PerformanceTimings {
        let total_time = start_time.elapsed().as_millis();
        PerformanceTimings {
            dns_lookup_time: 0,
            tcp_connect_time: 0,
            tls_handshake_time: 0,
            first_byte_time: 0,  // mor
            content_download_time: 0,
            ssl_negotiation_time: 0,
            ssl_cert_valid: true,
            request_sending_time: start_time_unix,
            server_processing_time: 0,
            response_receiving_time: total_time,
            total_time: total_time,
        }
    }
    // 拼接内容校验的内容
    fn get_content_verify_result(&self, status: bool, rule_type: ContentVerificationRules, rule_content: String, rule_description: String, true_rules: &mut Vec<ContentVerificationRulesResult>, failed_rules: &mut Vec<ContentVerificationRulesResult>) {
        let result =  ContentVerificationRulesResult {
            rule_id: Uuid::new_v4().as_u128() as u64,
            status: if status { StatusInfo::Success } else { StatusInfo::Failed },
            message: if status { "Rule verification passed".to_string() } else { "Rule verification failed".to_string() },
            rules: ContentVerificationRulesSingle{rule_type: rule_type.clone(),rule_content: rule_content.clone(),rule_description: rule_description.clone()}
        };
        if status {
            true_rules.push(result);
        } else {
            failed_rules.push(result);
        }
    }
    // 创建内容验证结果
    fn create_content_verification_result(&self,body: String, rules: &Option<Vec<ContentVerificationRulesSingle>>) -> ContentVerificationResult {
        if rules.is_none() || body.is_empty(){
            return ContentVerificationResult {
                match_rules: vec![],
                failed_rules: vec![],
            };
        }
        let rules = rules.as_ref().unwrap();
        let mut match_rules = vec![];
        let mut failed_rules = vec![];

        for ContentVerificationRulesSingle{rule_type, rule_content,rule_description } in rules.iter() {
           let status = match rule_type {
                ContentVerificationRules::Contains => {
                    body.contains(rule_content)
                },
                ContentVerificationRules::NotContains => {
                    !body.contains(rule_content)
                },
                ContentVerificationRules::Regex => {
                    let regex = Regex::new(&rule_content).unwrap();
                     regex.is_match(&body)
                },
                _ => {
                    false
                },
            };
            self.get_content_verify_result(status, rule_type.clone(), rule_content.clone(), rule_description.clone(), &mut match_rules, &mut failed_rules);

        }
        ContentVerificationResult {
            match_rules: match_rules,
            failed_rules: failed_rules,
        }
    }
    // 创建高级可用性结果
    fn create_advanced_availability_result(&self) -> AdvancedAvailability {
        AdvancedAvailability {
            bussiness_metrics: HashMap::new(),
        }
    }
    fn create_failed_result(&self, message: &str) -> CheckResult {
        CheckResult {
            id: uuid::Uuid::new_v4().as_u128(),
            monitor_type: MonitorType::Http,
            target: None,
            status: false,
            details: CheckResultDetail::Http(HttpMonitorResult {
                basic_avaliable: BasicAvailability {
                    is_reachable: false,
                    dns_resolvable: false,
                    tcp_connect_success: false,
                    res_received: false,
                    res_status_code: None,
                    res_status_category: StatusCategory::Unknown,
                    protocol_version: "".to_string(),
                    res_content_type: None,
                    res_content_length: 0,
                    res_charset: None,
                },
                ..Default::default()
            }),
        }
    }

}

// 这个地方需要严格按照Monitor trait要求实现的方法
#[async_trait::async_trait]
impl Monitor for HttpMonitor{
    async fn check(&self, config: &MonitorConfig) -> CheckResult {
         println!("Checking HTTP: {:?}", config.target);
         // 判断一下是否存在 target 此时的target 应该是一个 URL 地址
         if config.target.is_none() {
            return self.create_failed_result("Missing target URL");
         }
         let target = config.target.as_ref().unwrap();
         let url = match reqwest::Url::parse(target) {
            Ok(url) => url,
            Err(_) => return self.create_failed_result("Invalid target URL"),
         };

         match config.details {
             MonitorConfigDetail::Http(ref detail) => {
                // 获取一个http 链接
                 let request_client = self.get_client(detail);
                 let start_time_instant = std::time::Instant::now();
                 match request_client.send().await{
                    Ok(response)=> {
                            // 处理成功时的逻辑
                            // 获取状态码
                            let (dns_lookup_time, tcp_connect_time, tls_handshake_time, ssl_certificate_info) = match get_dns_tcp_tls_performance(config.target.as_ref().unwrap()).await {
                                Ok(performance) => performance,
                                Err(_) => (0, 0, 0, None),
                            };
                            // 提取需要的数据
                            let status_code = response.status().as_u16();
                            let version = response.version();
                            let headers = response.headers().clone();
                            let content_length = response.content_length();
                            let body = response.text().await.unwrap_or_default();
                            // 创建基本结果和头部结果
                            let basic_avaliable = self.create_basic_result(status_code, version, &headers, content_length);
                            let response_headers = self.create_headers_result(&headers);
                            let performance_timings = PerformanceTimings { dns_lookup_time, tcp_connect_time, tls_handshake_time, ..self.create_performance_timings(start_time_instant, unix_now_ms()) };
                            return CheckResult {
                                id: uuid::Uuid::new_v4().as_u128(), // V4 uuid 一种随机数 基于时间戳和随机数 V4版本的生成功能
                                monitor_type: MonitorType::Http,
                                target: config.target.clone(),
                                status: true, // 代表监控状态为成功，但是target访问失败了
                                details: CheckResultDetail::Http(HttpMonitorResult {
                                    basic_avaliable,
                                    response_headers,
                                    performance_timings,
                                    certificate_info: ssl_certificate_info.unwrap_or_default(),
                                    content_verification: self.create_content_verification_result(body, &detail.rules),
                                    advanced_avaliable: self.create_advanced_availability_result(),
                                })
                            }
                    },
                    Err(_) => {
                        // 检查各种错误类型
                        return CheckResult {
                            id: uuid::Uuid::new_v4().as_u128(), // V4 uuid 一种随机数 基于时间戳和随机数 V4版本的生成功能
                            monitor_type: MonitorType::Http,
                            target: config.target.clone(),
                            status: true, // 代表监控状态为成功，但是target访问失败了
                            details: CheckResultDetail::Http(HttpMonitorResult::default()),
                        };
                    },
                 }
             },
             _ => {
                // 如果不是 HTTP 监控类型，返回错误结果
                return CheckResult {
                    id: uuid::Uuid::new_v4().as_u128(), // V4 uuid 一种随机数 基于时间戳和随机数 V4版本的生成功能
                    monitor_type: MonitorType::Http,
                    target: None,
                    status: false, // 监控状态失败 根本就没有进入监控当中去
                    details: CheckResultDetail::Unknown(UnknownQueryResult {
                        description: "调用监控类型: HTTP, 请查实后再继续操作".to_string(),
                        query_type: MonitorType::Http,
                    }),
                };
            },
         }
    }

    fn get_type(&self) -> MonitorType {
        MonitorType::Http
    }

}
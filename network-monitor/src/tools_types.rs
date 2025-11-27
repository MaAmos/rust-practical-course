/// 公共的枚举类型 定义

use crate::{ monitor::types::{HttpMonitorConfig, MonitorConfig, MonitorConfigDetail, UnknownQueryConfig}};
use reqwest::header::{ HeaderMap, HeaderValue, HeaderName };
use std::collections::HashMap;
use crate::database::models::{MonitorConfigModel};
// http相关监控前置配置参数，用于确定HTTP监控的具体行为
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum HttpMethodTypes {
    #[serde(rename = "GET")] // 使用serde的rename属性来匹配JSON中的字符串 没办法直接匹配字符串，只能使用rename属性来匹配
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "CONNECT")]
    Connect,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum HttpBody {
    Text(String),          // 文本 body
    Binary(Vec<u8>),       // 二进制 body
    Json(serde_json::Value),           // JSON body（使用 serde_json::Value）
    Empty,                 // 空 body
}

// 定义监控类型的枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonitorType {
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "DNS")]
    Dns,
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "FTP")]
    Ftp,
    #[serde(rename = "TRACEROUTE")]
    Traceroute,
    #[serde(rename = "CPU")]
    Cpu,
    #[serde(rename = "MEMORY")]
    Memory,
    #[serde(rename = "DISK")]
    Disk,
    #[serde(rename = "PROCESS")]
    Process,
    #[serde(rename = "UNKNOWN")]
    Unknown,

}

impl std::fmt::Display for MonitorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MonitorType::Dns => write!(f, "DNS"),
            MonitorType::Http => write!(f, "HTTP"),
            MonitorType::Ftp => write!(f, "FTP"),
            MonitorType::Traceroute => write!(f, "TRACEROUTE"),
            MonitorType::Cpu => write!(f, "CPU"),
            MonitorType::Memory => write!(f, "MEMORY"),
            MonitorType::Disk => write!(f, "DISK"),
            MonitorType::Process => write!(f, "PROCESS"),
            MonitorType::Icmp => write!(f, "ICMP"),
            MonitorType::Tcp => write!(f, "TCP"),
            MonitorType::Udp => write!(f, "UDP"),
            MonitorType::Unknown => write!(f, "UNKNOWN"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}


// 定义一个struct 来保存http相关监控的最终结果参数结构体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct HttpMonitorResult{
    pub basic_avaliable: BasicAvailability,
    pub response_headers: SecurityHeaders,
    pub performance_timings: PerformanceTimings,
    pub certificate_info: CertificateInfo, // SSL证书信息
    pub content_verification: ContentVerificationResult,
    pub advanced_avaliable: AdvancedAvailability,

}

// 定义状态码对应的状态枚举类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StatusCategory {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
    Unknown,
}

// 给对应的枚举值 添加方法 来获取不同的状态码的实际状态

impl StatusCategory {
    pub fn from_status_code(status_code: u16) -> Self {
        match status_code {
            100..=199 => StatusCategory::Informational,
            200..=299 => StatusCategory::Success,
            300..=399 => StatusCategory::Redirection,
            400..=499 => StatusCategory::ClientError,
            500..=599 => StatusCategory::ServerError,
            _ => StatusCategory::Unknown,
        }
    }
}
impl Default for StatusCategory {
    fn default() -> Self {
        StatusCategory::Unknown  // 设置默认值为 Unknown
    }
}

// 定义一个struct 来保存基本的HTTP可用性信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BasicAvailability{
    pub is_reachable:  bool, // 是否可达
    pub dns_resolvable: bool, // DNS解析是否成功
    pub tcp_connect_success: bool, // TCP链接是否成功
    // HTTP相关
    pub res_received: bool, // 是否收到响应
    pub res_status_code: Option<u16>, // 响应状态码
    pub res_status_category: StatusCategory, // 响应状态码类别
    pub protocol_version: String, // HTTP协议版本（如HTTP/1.1, HTTP/2）
    pub res_content_type: Option<String>, // 响应内容类型
    pub res_content_length: u64, // 响应内容长度
    pub res_charset: Option<String>, // 响应字符集




}

// 创建一个枚举类型，表示性能指标的类别
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PerformanceTimings {
   // 性能指标相关
    pub dns_lookup_time: u128, // DNS查询时间，单位毫秒
    pub tcp_connect_time: u128, // TCP连接时间，单位毫秒
    pub tls_handshake_time: u128, // TLS握手时间，单位毫秒
    pub first_byte_time: u128, // 首字节时间，单位毫秒
    pub content_download_time: u128, // 内容下载时间，单位毫秒
    pub ssl_negotiation_time: u128, // SSL协商时间，单位毫秒
    pub ssl_cert_valid: bool, // SSL证书是否有效
    pub request_sending_time: u128, // 请求发送时间，单位毫秒
    pub server_processing_time: u128, // 服务器处理时间，单位毫秒
    pub response_receiving_time: u128, // 响应接收时间，单位毫秒
    pub total_time: u128, // 总时间，单位毫秒

}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CertificateInfo {
    pub issuer: Option<String>, // 证书颁发者
    pub subject: Option<String>, // 证书主题
    pub valid_from: Option<String>, // 证书有效期开始时间
    pub valid_until: Option<String>, // 证书有效期结束时间
    pub serial_number: Option<String>, // 证书序列号
    pub signature_algorithm: Option<String>, // 签名算法
    pub public_key_algorithm: Option<String>, // 公钥算法
    pub public_key_size: Option<usize>, // 公钥大小
    pub is_valid: bool, // 证书是否有效
}

// 定义安全头的枚举类型
pub enum SecurityHeaderType {

}


// 定义一个struct 来保存安全头监控信息
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SecurityHeaders{
    pub strict_transport_security: Option<String>, // 是否存在Strict-Transport-Security头
    pub content_security_policy: Option<String>, // 是否存在Content-Security-Policy头
    pub x_content_type_options: Option<String>, // 是否存在X-Content-Type-Options头
    pub x_frame_options: Option<String>, // 是否存在X-Frame-Options头
    pub x_xss_protection: Option<String>, // 是否存在X-XSS-Protection头
    pub referrer_policy: Option<String>, // 是否存在Referrer-Policy头
    pub feature_policy: Option<String>, // 是否存在Feature-Policy头
    pub permissions_policy: Option<String>, // 是否存在Permissions-Policy头
    pub security_headers_ok: bool, // 是否所有安全头都存在

}

// 创建一个枚举类型，表示高级可用性指标的类别

// 内容验证类别
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ContentVerificationRules {
    #[serde(rename = "contains")]
    Contains, // 响应内容中包含指定字符串
    #[serde(rename = "not_contains")]
    NotContains, // 响应内容中不包含指定字符串
    #[serde(rename = "regex")]
    Regex, // 响应内容匹配正则表达式
    // 定义一个默认值 用于匹配
    #[serde(rename = "default")] // 定义默认值
    Default,
}
// 设置默认值
impl Default for ContentVerificationRules {
    fn default() -> Self {
        ContentVerificationRules::Default
    }
}

//  创建一个内容验证结果
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ContentVerificationResult {
    pub match_rules: Vec<ContentVerificationRulesResult>, // 匹配的规则
    pub failed_rules: Vec<ContentVerificationRulesResult>, // 未匹配的规则 及其原因
}

// 高级可用性类别 业务指标监控 事务监控 等
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AdvancedAvailability {
    pub bussiness_metrics: HashMap<String, String>,
}


//定义内容监控规则结构体明细字段
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ContentVerificationRulesSingle {
    pub rule_type: ContentVerificationRules,
    pub rule_content: String,
    pub rule_description: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum StatusInfo {
    Success,
    Failed,
    Unknown,
}
impl Default for StatusInfo {
    fn default() -> Self {
        StatusInfo::Unknown
    }
}

// 定义核心内容验证返回数据结构体
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ContentVerificationRulesResult {
    pub rule_id: u64,
    pub status:StatusInfo,
    pub message: String,
    pub rules: ContentVerificationRulesSingle
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum AlertRuleTypes {
    #[serde(rename = "RESPONSE_CODE")]
    ResponseCode,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
impl Default for AlertRuleTypes {
    fn default() -> Self {
        AlertRuleTypes::Unknown
    }

}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AlertRuleDetail {
    pub rule_type: AlertRuleTypes,
    pub condition: NotifyCondition
}

// 定义告警规则结构体
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct AlertVerificationRules {
    pub notify_type: String,
    pub notify_config: NotifyConfig,
    pub rules: Vec<AlertRuleDetail>,

}
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct NotifyConfig {
    pub webhook_url: String
}
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct NotifyCondition{
    pub no_contains: Vec<u16>,
    pub contains: Vec<u16>,
    pub regex: String
}



#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct SelfDefineMonitorConfig {
    pub target: Option<String>,
    pub monitor_type: MonitorType,
    pub method: Option<HttpMethodTypes>,
    pub interval: Option<u64>,
    pub timeout: Option<u64>,
    pub params: Option<std::collections::HashMap<String, String>>,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub body: Option<HttpBody>,
    pub content_evaluation_rules: Option<Vec<ContentVerificationRulesSingle>>,
    pub alert_rules: Option<AlertVerificationRules>
}

// 定义转化的类型参数配置
pub fn convert_to_monitor_config(config: &MonitorConfigModel) -> MonitorConfig {
    // let config_json = serde_json::from_str::<SelfDefineMonitorConfig>(&config.config_json.clone().unwrap_or_default()).unwrap();
            // 构建配置的header 头
    let config_json = match serde_json::from_str::<SelfDefineMonitorConfig>(&config.config_json.clone().unwrap_or_default()) {
        Ok(json) => json,
        Err(_) => SelfDefineMonitorConfig { target: None, monitor_type: MonitorType::Unknown, method: None, interval: None, timeout: None, params: None, headers: None, body: None, content_evaluation_rules: None, alert_rules: None },
    };
    match config.monitor_type.as_str() {
        "HTTP"  => {
            let headers = config_json.headers.as_ref().map(|hdrs| {
                let mut header_map: HeaderMap = HeaderMap::new();
                for (key, value) in hdrs {
                    // 这个地方的key 因为是一个string 类型 需要转换成 HeaderName 类型 ，但是直接使用 HeaderName::from_str 会报错 因为没有这个方法，所以使用 parse方法 来转换
                    if let (Ok(header_name), Ok(header_value)) = (key.parse::<HeaderName>(), HeaderValue::from_str(&value)) {
                        header_map.insert(header_name, header_value);
                    }
                }
                header_map
            });
            let http_config = HttpMonitorConfig {
                url: config.target.clone(),
                method: config_json.method.clone().unwrap_or(HttpMethodTypes::Get), // 默认使用 GET 方法
                timeout: config_json.timeout.unwrap_or(10), // 默认超时时间10秒
                headers: headers.clone(),
                body: config_json.body.clone(),
                rules: config_json.content_evaluation_rules.clone(),
                alert_config: config_json.alert_rules.clone()

            };
            return MonitorConfig {
                target: Option::<String>::from(config.target.clone()),
                monitor_type: config_json.monitor_type.clone(),
                interval: config_json.interval, // 默认间隔60秒
                details: MonitorConfigDetail::Http(http_config),
            };
        },
        _ => {
            return MonitorConfig {
                target: Option::<String>::from(config.target.clone()),
                monitor_type: config_json.monitor_type.clone(),
                interval: config_json.interval,
                details: MonitorConfigDetail::Unknown(UnknownQueryConfig{
                    description: "Unsupported monitor type".to_string()
                }),
            };
        }


    }
}



// 定义不同的告警类型枚举值
// pub enum
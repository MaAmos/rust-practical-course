
use crate::tools_types::{HttpMethodTypes, MonitorType, HttpBody, HttpMonitorResult, ContentVerificationRulesSingle, AlertVerificationRules};


use reqwest::header::{ HeaderMap, HeaderValue };

// 定义核心的监控类型配置结构体
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub target: Option<String>, // 监控的目标URL或IP地址
    pub interval: Option<u64>, // 监控间隔，单位秒
    pub monitor_type: MonitorType,
    pub details: MonitorConfigDetail,
}

// 定义一个监控数据的详情监控参数 结构体 ，用于每个监控类型中的不同的参数类型
#[derive(Debug, Clone)]
pub enum MonitorConfigDetail {
    Http(HttpMonitorConfig),
    Https(HttpsMonitorConfig),
    Ftp(FtpMonitorConfig),
    Traceroute(TracerouteMonitorConfig),
    Cpu(CpuMonitorConfig),
    Memory(MemoryMonitorConfig),
    Disk(DiskMonitorConfig),
    Process(ProcessMonitorConfig),
    Unknown(UnknownQueryConfig),
}


#[derive(Debug, Clone)]
pub struct UnknownQueryConfig{
    pub description: String, // 异常描述信息
}





pub struct IcmpMonitorConfig{}
#[derive(Debug, Clone)]
pub struct TcpMonitorConfig{}
#[derive(Debug, Clone)]
pub struct UdpMonitorConfig{}
#[derive(Debug, Clone)]
pub struct DnsMonitorConfig{}


#[derive(Debug, Clone)]
pub struct FtpMonitorConfig{}
#[derive(Debug, Clone)]
pub struct HttpsMonitorConfig{}



#[derive(Debug, Clone)]
pub struct HttpMonitorConfig {
    pub url: String,
    pub method: HttpMethodTypes, // GET, POST, etc.
    pub timeout: u64, // 配置超时时间
    pub headers: Option<HeaderMap<HeaderValue>>, // 可选的HTTP头
    pub body: Option<HttpBody>,
    pub rules: Option<Vec<ContentVerificationRulesSingle>>,
    pub alert_config: Option<AlertVerificationRules>,
}



#[derive(Debug, Clone, Default)]
pub struct CpuMonitorConfig{}

#[derive(Debug, Clone, Default)]
pub struct MemoryMonitorConfig{}
#[derive(Debug, Clone, Default)]
pub struct DiskMonitorConfig{}

#[derive(Debug, Clone, Default)]
pub struct ProcessMonitorConfig{}


#[derive(Debug, Clone, Default)]
pub struct TracerouteMonitorConfig{}




// 定义检查结果结构体
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: u128, // 监控记录的ID 唯一标识
    pub monitor_type: MonitorType,
    pub target: Option<String>, // 监控的目标URL或IP地址
    pub status: bool, // 监控状态，true表示正常，false表示异常
    pub details: CheckResultDetail, // 监控结果的详细信息
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CheckResultDetail {
    Http(HttpMonitorResult),
    Https(HttpsMonitorResult),
    Ftp(FtpMonitorResult),
    Traceroute(TracerouteMonitorResult),
    Cpu(CpuMonitorResult),
    Memory(MemoryMonitorResult),
    Disk(DiskMonitorResult),
    Process(ProcessMonitorResult),
    Icmp(IcmpMonitorResult),
    Tcp(TcpMonitorResult),
    Udp(UdpMonitorResult),
    Dns(DnsMonitorResult),
    Unknown(UnknownQueryResult),
}


#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct HttpsMonitorResult {
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CpuMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MemoryMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DiskMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DnsMonitorResult {
}
// 未知查询类型配置结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnknownQueryResult{
    pub description: String, // 异常描述信息
    pub query_type: MonitorType, // 查询的监控类型
}

impl Default for UnknownQueryResult {
    fn default() -> Self {
        UnknownQueryResult {
            description: String::from("No description"),
            query_type: MonitorType::Unknown,
        }
    }
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TcpMonitorResult {
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UdpMonitorResult {
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct IcmpMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FtpMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TracerouteMonitorResult {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProcessMonitorResult {
}
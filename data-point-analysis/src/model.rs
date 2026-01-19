
// 定义Kafka数据模型
/**
 * {"@timestamp":"2025-11-25T09:12:24.502Z","@metadata":{"beat":"filebeat","type":"_doc","version":"7.4.0","topic":"analysislog_qas"},"ecs":{"version":"1.1.0"},"host":{"name":"IT-Q-OneCollect"},"agent":{"type":"filebeat","ephemeral_id":"424aa0ec-db34-41c2-b7dd-7c8c5d776388","hostname":"IT-Q-OneCollect","id":"b94b92f1-2bc0-4242-873e-fe7e57bceae6","version":"7.4.0"},"log":{"offset":66232501,"file":{"path":"/home/cttq/analysis.log"}},"message":"{\"project\":\"SAAS-GroupProcess-WebPC\",\"serverTime\":\"1764061944472\",\"data\":{\"identities\":{\"$identity_cookie_id\":\"197539fc92e1561-07a18dcd02bd358-26011e51-2073600-197539fc92f876\"},\"distinct_id\":\"197539fc92e1561-07a18dcd02bd358-26011e51-2073600-197539fc92f876\",\"lib\":{\"$lib\":\"js\",\"$lib_method\":\"code\",\"$lib_version\":\"1.24.14\"},\"properties\":{\"$timezone_offset\":-480,\"$screen_height\":720,\"$screen_width\":1280,\"$viewport_height\":621,\"$viewport_width\":1422,\"$lib\":\"js\",\"$lib_version\":\"1.24.14\",\"project\":\"SAAS-GroupProcess-WebPC\",\"projectName\":\"集团化流程中心\",\"projectId\":\"10062-SAAS\",\"personId\":\"8106152\",\"evtId\":\"10000001\",\"platform\":\"PC\",\"chromeVersion\":\"141.0.0.0\",\"safariVersion\":\"-\",\"lark\":\"-\",\"$referrer\":\"https://saasuser1qas.cttq.com/cvue/GroupProcessTest-WebPC/designCenter/customDesign?designDetail\\u003d1\\u0026schemeId\\u003d330374e5-4a69-4971-b87f-bd4a8dcf3202\\u0026id\\u003dce83ae8f-acf8-43e5-b1e7-f17fc9390535\\u0026type\\u003dview\",\"$url\":\"https://saasuser1qas.cttq.com/cvue/GroupProcessTest-WebPC/designCenter/processDesign?v\\u003d1764061941600\",\"$url_path\":\"/cvue/GroupProcessTest-WebPC/designCenter/processDesign\",\"$title\":\"\",\"$is_first_day\":false,\"$is_first_time\":false,\"$referrer_host\":\"saasuser1qas.cttq.com\"},\"anonymous_id\":\"197539fc92e1561-07a18dcd02bd358-26011e51-2073600-197539fc92f876\",\"type\":\"track\",\"event\":\"$pageview\",\"time\":1764061941628,\"_track_id\":213181628,\"_flush_time\":1764061941628},\"token\":\"d41b5b0c-1bcc-4917-be67-de0fa20cccb8\",\"ip\":\"10.81.86.195\"}","tags":["analysis","172.17.6.135","middle-app","qas","OneCollect","大前端组"],"input":{"type":"log"},"fields":{"log_topics":"analysislog_qas"}}
 *
 *
 *
 *
*/
use serde::{Deserialize, Serialize};
use serde_json::Value;
use clickhouse::{ Row};
use crate::utils::help_utils::{
    get_peak_hour,
    get_user_path_by_three,
    init_hour_histogram,
    is_night_cat,
    set_user_hist_config,
    set_user_device_hist,
    set_user_platform_hist,
    time_diff_to_day,
    update_hour_histogram};
// 外层 Filebeat/Kafka 日志载体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPointLog {
    #[serde(rename = "@timestamp")]
    pub timestamp: String,

    #[serde(rename = "@metadata")]
    pub metadata: Metadata,

    pub ecs: Ecs,
    pub host: Host,
    pub agent: Agent,
    pub log: LogInfo,

    // 内层事件JSON字符串（需二次反序列化）
    pub message: String,

    pub tags: Option<Vec<String>>,
    pub input: Option<InputInfo>,
    pub fields: Option<FieldsInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub beat: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub version: Option<String>,
    pub topic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ecs {
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub ephemeral_id: Option<String>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogInfo {
    pub offset: Option<i64>,
    pub file: Option<LogFile>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogFile {
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputInfo {
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldsInfo {
    pub log_topics: Option<String>,
}

// 内层事件（message 字符串反序列化后的结构）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InnerEvent {
    pub project: Option<String>,
    // 示例里是字符串时间戳，保留为 String；如需转 i64 可做自定义转换
    #[serde(rename = "serverTime")]
    pub server_time: Option<String>,

    pub data: Option<InnerData>,
    pub token: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InnerData {
    pub identities: Option<Value>, // 如需细分可建结构体
    pub distinct_id: Option<String>,
    pub lib: Option<LibInfo>,
    pub properties: Option<Properties>,
    pub anonymous_id: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub event: Option<String>,
    pub time: Option<i64>,
    pub _track_id: Option<i64>,
    pub _flush_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibInfo {
    #[serde(rename = "$lib")]
    pub lib: Option<String>,
    #[serde(rename = "$lib_method")]
    pub lib_method: Option<String>,
    #[serde(rename = "$lib_version")]
    pub lib_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "$timezone_offset")]
    pub timezone_offset: Option<i32>,
    #[serde(rename = "$screen_height")]
    pub screen_height: Option<i32>,
    #[serde(rename = "$screen_width")]
    pub screen_width: Option<i32>,
    #[serde(rename = "$viewport_height")]
    pub viewport_height: Option<i32>,
    #[serde(rename = "$viewport_width")]
    pub viewport_width: Option<i32>,
    #[serde(rename = "$lib")]
    pub lib: Option<String>,
    #[serde(rename = "$lib_version")]
    pub lib_version: Option<String>,
    #[serde(rename = "deviceUA")]
    pub usr_device: Option<String>,
    pub project: Option<String>,
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "personId")]
    pub person_id: Option<String>,
    #[serde(rename = "evtId")]
    pub evt_id: Option<String>,
    pub platform: Option<String>,
    #[serde(rename = "chromeVersion")]
    pub chrome_version: Option<String>,
    #[serde(rename = "safariVersion")]
    pub safari_version: Option<String>,
    pub lark: Option<String>,

    #[serde(rename = "$referrer")]
    pub referrer: Option<String>,
    #[serde(rename = "$url")]
    pub url: Option<String>,
    #[serde(rename = "$url_path")]
    pub url_path: Option<String>,
    #[serde(rename = "$title")]
    pub title: Option<String>,
    #[serde(rename = "$is_first_day")]
    pub is_first_day: Option<bool>,
    #[serde(rename = "$is_first_time")]
    pub is_first_time: Option<bool>,
    #[serde(rename = "$referrer_host")]
    pub referrer_host: Option<String>,
    pub event_duration: Option<u64>,
}

// 工具函数：从 OuterLog 解析出 InnerEvent
impl DataPointLog {
    pub fn parse_inner_event(&self) -> Result<InnerEvent, serde_json::Error> {
        serde_json::from_str::<InnerEvent>(&self.message)
    }
}


// 定义当前会话数据结构图
#[derive(Debug, Serialize, Deserialize, Clone, Row)]
pub struct SessionEventInfo {
    pub session_id: String,
    pub project_code: String,
    pub project_id: String,
    pub project_name: String,
    pub event_type: DataPointEventType,
    pub event_time: i64,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub viewport_width: Option<i32>,
    pub viewport_height: Option<i32>,
    pub person_code: String,
    pub event_id: String,
    pub event_platform: String,
    pub event_chrome_version: String,
    pub event_safari_version: String,
    pub event_cttq_lark: String,
    pub event_refferer: String,
    pub event_url: String,
    pub event_url_path: String,
    pub event_page_title: String,
    pub event_referrer_host: String,
    pub event_duration: Option<u64>,
    pub usr_device: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataPointEventType {
    // 页面浏览事件
    #[serde(rename = "$pageview")]
    Pageview,
    // 页面离开事件
    #[serde(rename = "$WebPageLeave")]
    WebPageLeave,
    // 自定义点击事件
    #[serde(rename = "countEvent")]
    CountEvent,
    // 曝光事件
    #[serde(rename = "Exposure")]
    Exposure,
    // 页面点击事件
    #[serde(rename = "$WebClick")]
    WebClick,
    // 未知事件
    #[serde(other)]
    Unknown,
}
impl DataPointEventType {
    pub fn from_str(str: &str) -> Self {
        match str {
            "$pageview" => Self::Pageview,
            "$WebPageLeave" => Self::WebPageLeave,
            "countEvent" => Self::CountEvent,
            "Exposure" => Self::Exposure,
            "$WebClick" => Self::WebClick,
            _ => Self::Unknown,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            DataPointEventType::Pageview => "$pageview",
            DataPointEventType::WebPageLeave => "$WebPageLeave",
            DataPointEventType::CountEvent => "countEvent",
            DataPointEventType::Exposure => "Exposure",
            DataPointEventType::WebClick => "$WebClick",
            DataPointEventType::Unknown => "Unknown",
        }
    }

}

// 定义存储redis中的信息数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisStorageEventInfo {
    pub session_id: String,
    pub person_id: String,
    pub last_activity_time: i64,
    pub event_type: DataPointEventType,
    pub project: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Row)]
pub struct SessionEventRecord {
    pub session_id: String,
    pub project_code: String,
    pub project_id: String,
    pub project_name: String,
    pub event_type: String,          // 原 enum 转为字符串
    pub event_time: i64,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub viewport_width: Option<i32>,
    pub viewport_height: Option<i32>,
    pub person_code: String,
    pub event_id: String,
    pub event_platform: String,
    pub event_chrome_version: String,
    pub event_safari_version: String,
    pub event_cttq_lark: String,
    pub event_refferer: String,
    pub event_url: String,
    pub event_url_path: String,
    pub event_page_title: String,
    pub event_referrer_host: String,
    pub event_duration: Option<i64>,
    pub usr_device: String,
}

impl From<SessionEventInfo> for SessionEventRecord {
    fn from(src: SessionEventInfo) -> Self {
        SessionEventRecord {
            session_id: src.session_id,
            project_code: src.project_code,
            project_id: src.project_id,
            project_name: src.project_name,
            event_type: src.event_type.as_str().to_string(),
            event_time: src.event_time,
            screen_width: src.screen_width,
            screen_height: src.screen_height,
            viewport_width: src.viewport_width,
            viewport_height: src.viewport_height,
            person_code: src.person_code,
            event_id: src.event_id,
            event_platform: src.event_platform,
            event_chrome_version: src.event_chrome_version,
            event_safari_version: src.event_safari_version,
            event_cttq_lark: src.event_cttq_lark,
            event_refferer: src.event_refferer,
            event_url: src.event_url,
            event_url_path: src.event_url_path,
            event_page_title: src.event_page_title,
            event_referrer_host: src.event_referrer_host,
            event_duration: src.event_duration.map(|d| d as i64),
            usr_device:src.usr_device,
        }
    }
}


// 保存一份 存储在 sqlite 中的数据结构 包含每次会话的各种特征值 方便后续的数据聚合和分析
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionFeatureInfo {
    pub session_id: String,
    pub person_id: String,
    pub project_code: String,
    pub start_time: i64,
    pub end_time: i64,
    pub duration_secs: u64,
    pub total_events: u64,
    pub pageview_count: u64,
    pub click_count: u64,
    pub exposure_count: u64,
    pub web_page_leave_count: u64,
    pub count_event_count: u64,
    pub usr_device: String,
}

// 特征提取 结构体 用于后续的模型聚合和分析 将低维度的会话数据 转换为 高维度的特征向量
// 基础特征 直接从事件中取
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicFeatureVector {
    pub event_type: DataPointEventType, // 存储当前会话中的事件类型
    pub timestamp_secs: i64, // 发生时间戳
    pub page_category: String, // 页面类别
    pub device_type: String, // 设备类型
    pub event_duration: Option<u64>, // 事件持续时间
    pub device_version: String, // 设备版本
}

// 会话级特征 聚合单次会话
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionFeatureVector {
    pub session_druation_secs: u64, // 会话持续时间
    pub pageview_count: u64, // 页面浏览事件数量
    pub click_density: f64, // 点击密度 计算公式：点击事件数量 / 页面浏览事件数量
    pub bounce: bool, // 跳出率 计算公式：仅浏览一个页面后离开的会话数量 / 总会话数量
    pub conversion_rate: bool, // 转化率 计算公式：完成特定目标的会话数量 / 总会话数量
    pub path_complexity: f64, // 路径复杂度 计算公式：会话中独特页面数量 / 会话中总页面数量
    pub page_per_page: Vec<f64>, // 每页面停留时间
}

// 用户级特征 聚合多条会话
#[derive(Debug, Serialize, Deserialize, Clone, Row)]
pub struct UserFeatureVector {
    pub user_id: String, // 用户ID 工号
    pub timestamp: i64, // 当前时间戳

    // 基础特征
    pub total_sessions: u64, // 总会话数量   计算公式：该用户下的所有会话数量
    pub total_events: u64, // 总事件数量  计算公式：该用户下的所有事件类型去重汇总的数量
    pub total_pageviews: u64, // 总页面浏览事件数量 计算公式：该用户下的所有访问页面数量
    pub total_clicks: u64, // 总点击事件数量 计算公式：该用户下的所有点击事件数量
    pub total_exposures: u64, // 总曝光事件数量 计算公式：该用户下的所有曝光事件数量
    pub total_web_page_leaves: u64, // 总网页离开事件数量 计算公式：该用户下的所有网页离开事件数量
    pub total_pageview_duration_secs: u64, // 总页面停留时长 计算公式：该用户下的所有页面停留时长

    // 频率特征
    pub avg_sessions_per_day: f64, // 平均每日会话次数 计算公式：总会话数量 / 总天数
    pub avg_session_duration_secs: f64, // 平均会话持续时间 计算公式：总会话持续时间 / 总会话数量
    pub avg_events_per_session: f64, // 平均每会话事件数量 计算公式：总事件数量 / 总会话数量
    pub avg_pageviews_per_session: f64, // 平均每会话页面浏览事件数量 计算公式：总页面浏览事件数量 / 总会话数量
    pub avg_clicks_per_session: f64, // 平均每会话点击事件 计算公式：总点击事件数量 / 总会话数量
    pub avg_exposures_per_session: f64, // 平均每会话曝光事件 计算公式：总曝光事件数量 / 总会话数量
    pub avg_web_page_leaves_per_session: f64, // 平均每会话网页离开事件 计算公式：总网页离开事件数量 / 总会话数量
    pub avg_count_events_per_session: f64, // 平均每会话计数事件 计算公式：总计数事件数量 / 总会话数量
    pub avg_session_depth: f64, // 平均会话深度（页面浏览数量） 计算公式：总页面浏览事件数量 / 总会话数量

    // 时间模式
    pub hours_hist: Vec<(i64, i64)>,
    pub preferred_hour: u8, // 用户偏好的活跃时间段（小时）
    pub is_night_owl: bool, // 是否为夜猫子用户（晚上活跃）

    // 内容偏好
    pub path_hist: Vec<(String, i64)>,
    pub favorite_page_categories: Vec<String>, // 用户最爱访问的页面类别
    pub user_device_hist: Vec<(String, i64)>, // 设置用户设备的访问频率 设备类型
    pub user_platform_hist: Vec<(String, i64)>, // 设置用户平台访问频率 （PC/Android/iOS_version, count）
    pub favorite_device_types: Vec<String>, // 用户最爱访问的设备类型
    pub search_history: Vec<String>, // 用户搜索历史关键词

    // 版本偏好

    pub preferred_lark_type: String,
    pub user_lark_hist: Vec<(String, i64)>, // 用户lark版本访问频率
}

impl UserFeatureVector {
    pub fn new(person_code: String) -> Self {
        UserFeatureVector {
            user_id: person_code,
            timestamp: 0,
            total_sessions: 0,
            total_events: 0,
            total_pageviews: 0,
            total_clicks: 0,
            total_exposures: 0,
            total_web_page_leaves: 0,
            total_pageview_duration_secs: 0,
            avg_sessions_per_day: 0.0,
            avg_session_duration_secs: 0.0,
            avg_events_per_session: 0.0,
            avg_pageviews_per_session: 0.0,
            avg_clicks_per_session: 0.0,
            avg_exposures_per_session: 0.0,
            avg_web_page_leaves_per_session: 0.0,
            avg_count_events_per_session: 0.0,
            avg_session_depth: 0.0,
            hours_hist: init_hour_histogram(), // 初始化24小时直方图
            preferred_hour: 0,
            is_night_owl: false,
            path_hist: vec![], // 全量路径得分

            favorite_page_categories: vec![],
            user_device_hist: vec![],
            user_platform_hist: vec![],
            favorite_device_types: vec![],
            search_history: vec![],
            preferred_lark_type: "".to_string(),
            user_lark_hist: vec![],
        }
    }
    pub fn update_with_session(&mut self, session: &SessionEventInfo) {
        self.total_sessions += 1;
        self.total_events += 1; // 假设每个session至少有一个事件
        match session.event_type {
            DataPointEventType::Pageview => self.total_pageviews += 1,
            DataPointEventType::WebClick => self.total_clicks += 1,
            DataPointEventType::Exposure => self.total_exposures += 1,
            DataPointEventType::WebPageLeave => self.total_web_page_leaves += 1,
            DataPointEventType::CountEvent => self.total_clicks += 1,
            _ => {}
        }
        if let Some(duration) = session.event_duration {
            self.total_pageview_duration_secs += duration;
        }
        // 获取第一次session 到现在的天数
        let days_since_first_session = if self.timestamp <= 0 {
            self.timestamp = session.event_time;
            1 } else { let diff = time_diff_to_day(self.timestamp, session.event_time);
                if diff == 0 { 1 } else { diff }
             };
        self.avg_sessions_per_day = self.total_sessions as f64 / days_since_first_session as f64;
        self.avg_session_duration_secs = self.total_sessions as f64 / days_since_first_session as f64;
        self.avg_events_per_session = self.total_events as f64 / self.total_sessions as f64;
        self.avg_pageviews_per_session = self.total_pageviews as f64 / self.total_sessions as f64;
        self.avg_clicks_per_session = self.total_clicks as f64 / self.total_sessions as f64;
        self.avg_exposures_per_session = self.total_exposures as f64 / self.total_sessions as f64;
        self.avg_web_page_leaves_per_session = self.total_web_page_leaves as f64 / self.total_sessions as f64;
        self.avg_count_events_per_session = self.total_clicks as f64 / self.total_sessions as f64;
        self.avg_session_depth = self.total_pageviews as f64 / self.total_sessions as f64;
        update_hour_histogram(&mut self.hours_hist, session.event_time);
        self.preferred_hour = get_peak_hour(&self.hours_hist);
        self.is_night_owl = is_night_cat(&self.hours_hist);
        self.path_hist = set_user_hist_config(&mut self.path_hist, session.event_url_path.clone());
        self.favorite_page_categories = get_user_path_by_three(self.path_hist.clone());
        self.user_device_hist = set_user_device_hist(&mut self.user_device_hist, session.usr_device.clone());
        self.user_platform_hist = set_user_platform_hist(&mut self.user_platform_hist, (session.event_platform.clone(), session.event_chrome_version.clone(), session.event_safari_version.clone()));
        self.favorite_device_types = get_user_path_by_three(self.user_device_hist.clone());
        self.search_history = Vec::new();
        self.user_lark_hist = set_user_hist_config(&mut self.user_lark_hist, session.event_cttq_lark.clone());
        self.preferred_lark_type = get_user_path_by_three(self.user_lark_hist.clone()).get(0).cloned().unwrap_or("unknown".to_string());

    }
}




use std::collections::HashMap;
use crate::model::{SessionEventInfo, InnerEvent,DataPointEventType, RedisStorageEventInfo};
use crate::storage::redis_client::RedisClient;
use crate::utils::error::{AppError, AppResult};
type SessionCheckResult = (bool, Option<String>, Option<String>, Option<String>, Option<i64>);
use uuid::Uuid;
use crate::storage::clickhouse::ClickhouseClient;
use super::feature_extractor::FeatureExtractor;
use log::{info, error, debug, warn};

// 定义会话分割器，根据当前的redis中的用户最后活动时间，判断是否需要开启新的会话
pub struct Sessionizer<'a> {
    redis_client: RedisClient,
    session_timeout_secs: u64,
    split_session_list: HashMap<String, Vec<SessionEventInfo>>,
    clickhouse_client: &'a ClickhouseClient,
    feature_extractor: FeatureExtractor<'a>,
}


impl<'a> Sessionizer<'a>{
    pub fn new(redis_client: RedisClient, session_timeout_secs: u64, clickhouse_client: &'a ClickhouseClient) -> Self {
        // 要存储当前redis中的用户最后活动时间 用于判断会话是否超时
        Sessionizer {
            redis_client,
            session_timeout_secs,
            split_session_list: HashMap::new(),
            clickhouse_client,
            feature_extractor: FeatureExtractor::new(&clickhouse_client),
        }
    }
    // 校验当前信息的必填字段是否有值，
    pub fn check_session_required_fields(&self,event_message: &InnerEvent) -> SessionCheckResult {
        // 必填字段 使用__来拆分不同层级的数据字段
        let required_fields = vec!["data__properties__person_id", "data__event", "project", "data__time"];
        let mut result: SessionCheckResult = (true, None, None, None, None);
        // 遍历必填字段
        for field in required_fields.iter() {
            // 使用__来拆分字段
            let ok = match *field {
                "data__properties__person_id" => {
                    match event_message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.person_id.as_ref()) {
                        Some(id) => { if id.len() > 0 { result.1 = Some(id.clone()); true } else { false } },
                        None => false,
                    }
                },
                "data__event" => {
                    match event_message.data.as_ref().and_then(|d| d.event.as_ref()) {
                        Some(e) => { result.2 = Some(e.clone()); true },
                        None => false,
                    }
                },
                "project" => {
                    match event_message.project.as_ref() {
                        Some(p) => { result.3 = Some(p.clone()); true },
                        None => false,
                    }
                },
                "data__time" => {
                    match event_message.data.as_ref().and_then(|d| d.time.as_ref()) {
                        Some(t) => { result.4 = Some(t.clone()); true },
                        None => false,
                    }
                },
                _ => true,
            };
            if !ok {
                result.0 = false;
            }

        }
        result
    }
    pub async fn create_new_session(&mut self,message: &InnerEvent, basic_info: SessionCheckResult, redis_session_key: &str, session_id: String)-> AppResult<bool> {
        let (_, person_id_opt, event_opt, project_opt, time_opt) = basic_info;
        // 创建新的会话
        let new_session_info = SessionEventInfo {
            session_id: session_id,
            project_code: project_opt.clone().unwrap_or("".to_string()),
            project_id: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.project_id.clone()).unwrap_or("".to_string()),
            project_name: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.project_name.clone()).unwrap_or("".to_string()),
            event_type: DataPointEventType::from_str(event_opt.as_ref().unwrap_or(&"".to_string())),
            event_time: time_opt.as_ref().and_then(|t| Some(*t)).unwrap_or(0),
            screen_width: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.screen_width),
            screen_height: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.screen_height),
            viewport_width: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.viewport_width),
            viewport_height: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.viewport_height),
            person_code: person_id_opt.clone().unwrap_or("".to_string()),
            event_id: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.evt_id.clone()).unwrap_or("".to_string()),
            event_platform: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.platform.clone()).unwrap_or("".to_string()),
            event_chrome_version: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.chrome_version.clone()).unwrap_or("".to_string()),
            event_safari_version: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.safari_version.clone()).unwrap_or("".to_string()),
            event_cttq_lark: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.lark.clone()).unwrap_or("".to_string()),
            event_refferer: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.referrer.clone()).unwrap_or("".to_string()),
            event_url: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.url.clone()).unwrap_or("".to_string()),
            event_url_path: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.url_path.clone()).unwrap_or("".to_string()),
            event_page_title: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.title.clone()).unwrap_or("".to_string()),
            event_referrer_host: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.referrer_host.clone()).unwrap_or("".to_string()),
            event_duration: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.event_duration),
            usr_device: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.usr_device.clone()).unwrap_or("".to_string()),
        };
        // 暂时存储在当前的结构体中，超过1000条？或者 什么条件下 进行批量存储到clickhouse中
        self.split_session_list
            .entry(person_id_opt.clone().unwrap_or("".to_string()))
            .or_insert_with(Vec::new)
            .push(new_session_info.clone());
        // 存储到redis中
        self.redis_client.set_json_value(&redis_session_key, &RedisStorageEventInfo{
            session_id: new_session_info.session_id.clone(),
            person_id: person_id_opt.clone().unwrap_or("".to_string()),
            last_activity_time:time_opt.as_ref().and_then(|t| Some(*t)).unwrap_or(0),
            event_type: DataPointEventType::from_str(event_opt.as_ref().unwrap_or(&"".to_string())),
            project: project_opt.clone().unwrap_or("".to_string()),
        }).map_err(|e| AppError::RedisError(redis::RedisError::from((redis::ErrorKind::Io, "Failed to set value to Redis", e.to_string()))))?;

        Ok(true)
    }
    pub async fn split_message_to_session(&mut self,message: &InnerEvent) -> AppResult<Option<SessionEventInfo>> {
        let (valid, person_id_opt, event_opt, project_opt, time_opt) = self.check_session_required_fields(&message);
        if !valid {
             return Ok(None);
            //return Err(AppError::InvalidDataError("Missing required fields for session splitting".to_string()));
        }
        // 获取当前的用户redis中的key
        let redis_session_key = format!("session:{}:last_active", person_id_opt.as_ref().unwrap_or(&"".to_string()));
        // 首先 获取redis中的用户最后活动时间 如果不存在 创建新的会话 ，如果存在的话 判断时间差是否超过session_timeout_secs 如果超过则创建新的会话
        let session_value = match self.redis_client.get_json_value::<RedisStorageEventInfo>(&redis_session_key) {
            Ok(val) => val,
            Err(e) => {
                return Err(AppError::RedisError(redis::RedisError::from((redis::ErrorKind::Io, "Failed to get value from Redis", e.to_string()))));
            }
        };

        // 超过session_timeout_secs设置的时间 则创建新的会话 如果没有用户会话 则创建新会话
        let create_session_id = if session_value.as_ref().is_none() {
            Uuid::new_v4().to_string()
        } else {
            let (last_activity_time_in_redis, session_id) = session_value.as_ref().map(|s| (s.last_activity_time, s.session_id.clone())).unwrap_or((0, Uuid::new_v4().to_string()));
            let current_event_time = time_opt.as_ref().and_then(|t| Some(*t)).unwrap_or(0);
            if current_event_time - last_activity_time_in_redis > self.session_timeout_secs as i64 {
                Uuid::new_v4().to_string()
            } else {
                session_id
            }
        };

        let (_, person_id_opt_val, event_opt_val, project_opt_val, time_opt_val) = (valid, person_id_opt.clone(), event_opt.clone(), project_opt.clone(), time_opt.clone());

        // --- Added: Return the SessionEventInfo for aggregation ---
        let new_session_info = SessionEventInfo {
            session_id: create_session_id.clone(),
            project_code: project_opt_val.clone().unwrap_or("".to_string()),
            project_id: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.project_id.clone()).unwrap_or("".to_string()),
            project_name: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.project_name.clone()).unwrap_or("".to_string()),
            event_type: DataPointEventType::from_str(event_opt_val.as_ref().unwrap_or(&"".to_string())),
            event_time: time_opt_val.as_ref().and_then(|t| Some(*t)).unwrap_or(0),
            screen_width: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.screen_width),
            screen_height: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.screen_height),
            viewport_width: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.viewport_width),
            viewport_height: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.viewport_height),
            person_code: person_id_opt_val.clone().unwrap_or("".to_string()),
            event_id: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.evt_id.clone()).unwrap_or("".to_string()),
            event_platform: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.platform.clone()).unwrap_or("".to_string()),
            event_chrome_version: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.chrome_version.clone()).unwrap_or("".to_string()),
            event_safari_version: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.safari_version.clone()).unwrap_or("".to_string()),
            event_cttq_lark: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.lark.clone()).unwrap_or("".to_string()),
            event_refferer: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.referrer.clone()).unwrap_or("".to_string()),
            event_url: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.url.clone()).unwrap_or("".to_string()),
            event_url_path: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.url_path.clone()).unwrap_or("".to_string()),
            event_page_title: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.title.clone()).unwrap_or("".to_string()),
            event_referrer_host: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.referrer_host.clone()).unwrap_or("".to_string()),
            event_duration: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.event_duration),
            usr_device: message.data.as_ref().and_then(|d| d.properties.as_ref()).and_then(|p| p.usr_device.clone()).unwrap_or("".to_string()),
        };

        // Reuse existing method but it reconstructs the object.
        // To avoid code duplication and complexity, I'll inline the logic of create_new_session effectively or call it.
        // But create_new_session puts it in split_session_list.
        // Let's call create_new_session as before, it works fine.

        match self.create_new_session(message, (valid, person_id_opt.clone(), event_opt.clone(), project_opt.clone(), time_opt.clone()), &redis_session_key, create_session_id).await {
            Ok(_) => {},
            Err(e) => {
                return Err(AppError::RedisError(redis::RedisError::from((redis::ErrorKind::Io, "Failed to create new session", e.to_string()))));
            }
        };

        Ok(Some(new_session_info))
    }
    pub async fn save_sessions_to_clickhouse(&self) -> AppResult<()> {
        // 遍历当前的split_session_list 将数据批量存储到clickhouse中
        for (person_id, sessions) in self.split_session_list.iter() {

            println!("Storing {} sessions for person_id: {}", sessions.len(), person_id);
            // 批量存储用户的会话数据
            self.clickhouse_client.insert_events_batch(sessions.clone()).await.unwrap_or_else(|e| {
                error!("Error insert_events_batch sessions to ClickHouse: {:?}", e);
            });
            // 批量提取相关用户的特征信息 并进行存储
            let user_feature = self.feature_extractor.extract_features(sessions.clone()).await?;
            self.clickhouse_client.update_user_features(user_feature).await.unwrap_or_else(|e| {
                error!("Error update_user_features sessions to ClickHouse: {:?}", e);
            });;
        }
        Ok(())
    }
}
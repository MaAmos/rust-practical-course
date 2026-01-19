// 特征提取器 基于会话的数据 进行提取相关用户的特征信息

use crate::model::{SessionEventInfo, UserFeatureVector};
use std::collections::HashMap;
use crate::utils::error::{AppError, AppResult};
use crate::storage::clickhouse::ClickhouseClient;
pub struct FeatureExtractor<'a> {
    stroage_features:  HashMap<String,  UserFeatureVector>,
    clickhouse_client: &'a ClickhouseClient,
}

impl<'a> FeatureExtractor<'a> {
    pub fn new(clickhouse_client: &'a ClickhouseClient) -> Self {
        FeatureExtractor {
            stroage_features: HashMap::new(),
            clickhouse_client,
        }
    }
    pub async fn extract_features(&self,session_list: Vec<SessionEventInfo>) -> AppResult<UserFeatureVector> {
       // 获取当前用户的特征信息
       let person_code = session_list[0].person_code.clone();
       let mut user_feature = match self.stroage_features.get(&person_code) {
           Some(features) => features.clone(),
           None => {
                // 这里的逻辑是 如果没有就去click house中 查询是否包含当前的用户的特征信息 如果有就返回 没有就返回一个空的
                match self.clickhouse_client.get_user_features(person_code.clone()).await {
                    Ok(features) => features,
                    Err(_) => UserFeatureVector::new(person_code.clone()),
                }
            }
       };
       // 进行特征提取
       for session in session_list {
           user_feature.update_with_session(&session);
       }
        Ok(user_feature)
    }
}
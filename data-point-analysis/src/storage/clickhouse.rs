use clickhouse::{Client, Row};
use crate::config::AppConfig;
use crate::utils::error::{AppError, AppResult};
use std::sync::Arc;
use crate::model::{SessionEventInfo,SessionEventRecord, UserFeatureVector};
use serde::{Deserialize, Serialize};
#[derive(Clone)]
pub struct ClickhouseClient {
    client: Arc<Client>,
}

impl ClickhouseClient {
    pub fn new(cfg: &AppConfig) -> AppResult<Self> {
        // let clickhouse_url = format!(
        //     "{}?database={}&username={}&password={}&compression={}&connection_timeout={}",
        //     cfg.storage.clickhouse.url,
        //     cfg.storage.clickhouse.database,
        //     cfg.storage.clickhouse.username,
        //     cfg.storage.clickhouse.password,
        //     cfg.storage.clickhouse.compression,
        //     cfg.storage.clickhouse.connection_timeout_secs
        // );
        let client = Client::default().with_url(cfg.storage.clickhouse.url.as_str())
            .with_user(cfg.storage.clickhouse.username.as_str())
            .with_password(cfg.storage.clickhouse.password.as_str())
            .with_database(cfg.storage.clickhouse.database.as_str());
        Ok(Self { client: Arc::new(client) })
    }

    // CREATE - 创建表
    pub async fn create_session_events_table(&self) -> AppResult<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS session_events (
                session_id String,
                project_code String,
                project_id String,
                project_name String,
                event_type String,
                event_time Int64,
                screen_width Nullable(Int32),
                screen_height Nullable(Int32),
                viewport_width Nullable(Int32),
                viewport_height Nullable(Int32),
                person_code String,
                event_id String,
                event_platform String,
                event_chrome_version String,
                event_safari_version String,
                event_cttq_lark String,
                event_refferer String,
                event_url String,
                event_url_path String,
                event_page_title String,
                event_referrer_host String,
                event_duration Nullable(Int64),
                usr_device String
            ) ENGINE = MergeTree()
            ORDER BY (session_id, event_time)
        "#;

        self.client.query(sql)
            .execute()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to create table: {}", e)))?;

        Ok(())
    }

    // INSERT - 插入单条数据
    pub async fn insert_event(&self, event: SessionEventInfo) -> AppResult<()> {
        let mut insert = self.client.insert::<SessionEventRecord>("session_events").await?;
        insert.write(&SessionEventRecord::from(event)).await?;
        insert.end().await?;

        Ok(())
    }

    // INSERT - 批量插入数据
    pub async fn insert_events_batch(&self, events: Vec<SessionEventInfo>) -> AppResult<()> {
        let mut insert = self.client.insert::<SessionEventRecord>("session_events").await?;
        for (idx,event) in events.into_iter().enumerate() {
            let record = SessionEventRecord::from(event);
            // 处理event中的数据格式
            insert.write(&record).await.map_err(|e| AppError::StorageError(format!("Failed to write event: {}", e)))?;
        }
        insert.end().await?;

        Ok(())
    }

    // SELECT - 查询数据
    pub async fn query_events_by_user(&self, user_id: &str, size: u32, limit: u32) -> AppResult<Vec<SessionEventInfo>> {
        let sql = "SELECT ?fields FROM session_events WHERE user_id = ? AND timestamp >= ?";
        let events: Vec<SessionEventInfo> = self.client
            .query(sql)
            .bind(user_id)
            .bind(size)
            .bind(limit)
            .fetch_all()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to query events: {}", e)))?;

        Ok(events)
    }

    // SELECT - 根据ID查询单条数据
    pub async fn get_event_by_session_id(&self, id: String) -> AppResult<Option<SessionEventInfo>> {
        let sql = "SELECT ?fields FROM session_events WHERE session_id = ? LIMIT 1";

        let events: Vec<SessionEventInfo> = self.client
            .query(sql)
            .bind(id)
            .fetch_all()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to get event by ID: {}", e)))?;

        Ok(events.into_iter().next())
    }

    // SELECT - 统计查询
    pub async fn count_events_by_project(&self, project: &str) -> AppResult<u64> {
        let sql = "SELECT COUNT(*) as count FROM session_events WHERE project = ?";

        #[derive(Row, Debug, Serialize, Deserialize)]
        struct CountResult {
            count: u64,
        }

        let result: Vec<CountResult> = self.client
            .query(sql)
            .bind(project)
            .fetch_all()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to count session_events: {}", e)))?;

        Ok(result.first().map(|r| r.count).unwrap_or(0))
    }

    // UPDATE - ClickHouse不支持传统UPDATE，但可以通过替换方式实现
    pub async fn replace_event(&self, event: SessionEventInfo) -> AppResult<()> {
        // 删除旧记录
        let delete_sql = "ALTER TABLE session_events DELETE WHERE session_id = ?";
        self.client
            .query(delete_sql)
            .bind(event.session_id.clone())
            .execute()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to delete old event: {}", e)))?;

        // 插入新记录
        self.insert_event(event).await?;

        Ok(())
    }

    // DELETE - 删除数据
    pub async fn delete_event_by_session_id(&self, id: String) -> AppResult<()> {
        let sql = "ALTER TABLE session_events DELETE WHERE session_id = ?";

        self.client
            .query(sql)
            .bind(id)
            .execute()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to delete event: {}", e)))?;

        Ok(())
    }

    // DELETE - 根据条件删除数据
    pub async fn delete_events_by_user(&self, user_id: &str) -> AppResult<()> {
        let sql = "ALTER TABLE session_events DELETE WHERE user_id = ?";

        self.client
            .query(sql)
            .bind(user_id)
            .execute()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to delete events by user: {}", e)))?;

        Ok(())
    }

    // 获取当前的用户的特征信息
    pub async fn get_user_features(&self, person_code: String) -> AppResult<UserFeatureVector> {
        let sql = "SELECT * FROM user_feature_vector WHERE user_id = ?";
        let features: Vec<UserFeatureVector> = self.client
            .query(sql)
            .bind(person_code.clone())
            .fetch_all()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to get user features: {}", e)))?;
        Ok(features.into_iter().next().unwrap_or_else(|| UserFeatureVector::new(person_code.clone())))
    }
    // 插入用户的特征信息
    pub async fn insert_features_batch(&self, user_feature: UserFeatureVector) -> AppResult<()> {
        println!("Inserting user features: {:?}", user_feature);
        let mut insert = self.client.insert::<UserFeatureVector>("user_feature_vector").await?;
        insert.write(&user_feature).await.map_err(|e| AppError::StorageError(format!("Failed to write user features: {}", e)))?;
        insert.end().await?;
        Ok(())
    }
    // 更新用户的特征信息
    pub async fn update_user_features(&self, user_feature: UserFeatureVector) -> AppResult<()> {
        // 先判断是否存在 如果存在则删除后插入新的 如果不存在则直接插入
        match self.get_user_features(user_feature.user_id.clone()).await {
            Ok(existing_feature) => {
                if existing_feature.user_id.is_empty() {
                    // 不存在 直接插入
                    self.insert_features_batch(user_feature).await?;
                    return Ok(());
                }else {
                    // 存在 则进行删除后插入
                    // 删除旧记录
                    let delete_sql: &str = "ALTER TABLE user_feature_vector DELETE WHERE user_id = ?";
                    self.client
                        .query(delete_sql)
                        .bind(user_feature.user_id.clone())
                        .execute()
                        .await
                        .map_err(|e| AppError::StorageError(format!("Failed to delete old user features: {}", e)))?;
                    // 插入新记录
                    self.insert_features_batch(user_feature).await?;
                }
            },
            Err(_) => {
                // 出错则认为不存在 直接插入
                self.insert_features_batch(user_feature).await?;
                return Ok(());
            }
        };
        Ok(())
    }

    // 获取客户端引用（用于复杂查询）
    pub fn get_client(&self) -> &Client {
        &self.client
    }

}
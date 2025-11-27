use diesel::prelude::*;
use crate::database::connect_db::{SqlitePool, get_connection};
use crate::database::models::{NotificationChannelsModel};
use crate::database::schema::{notification_channels};

pub struct NotificationChannelsRepository {
    pool: SqlitePool,
}

impl NotificationChannelsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        NotificationChannelsRepository { pool }
    }
    // 获取全部的提醒通道
    pub fn get_all_notification_channels(&self,limit_n: i64, offset: i64) -> Result<Vec<NotificationChannelsModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        notification_channels::table
            .order(notification_channels::created_at.desc())
            .limit(limit_n)
            .offset(offset)
            .load::<NotificationChannelsModel>(&mut conn)
    }
    // 根据channel_type获取通知数据
    pub fn get_notification_channel_by_type(&self, channel_type: String) -> Result<Vec<NotificationChannelsModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        notification_channels::table
            .filter(notification_channels::channel_type.eq(channel_type))
            .order(notification_channels::created_at.desc())
            .load::<NotificationChannelsModel>(&mut conn)
    }

    // 根据ID 来获取相关的通知数据
    pub fn get_notification_channel_by_id(&self, id: i32) -> Result<Vec<NotificationChannelsModel>, diesel::result::Error> {
        let mut conn = get_connection(&self.pool);
        notification_channels::table
            .filter(notification_channels::id.eq(id))
            .order(notification_channels::created_at.desc())
            .load::<NotificationChannelsModel>(&mut conn)

    }
}
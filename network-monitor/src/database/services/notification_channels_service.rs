use crate::database::repositories::notification_channels_repo::NotificationChannelsRepository;
use crate::database::models::{NotificationChannelsModel};

pub struct NotificationChannelsService {
    // 以后有别的仓库再加：repo: NotificationChannelsRepository,
    repo: NotificationChannelsRepository
}
impl NotificationChannelsService {
    pub fn new( repo: NotificationChannelsRepository) -> Self {
        NotificationChannelsService { repo }
    }
    pub fn get_notification_channel_by_type(&self, channel_type: String) -> Result<Vec<NotificationChannelsModel>, diesel::result::Error>{
        self.repo.get_notification_channel_by_type(channel_type)
    }
}
use channel::event::SWARM_SENDER;
use chat::chat::{ChatEvent, ChatReq};
use error::error::AppError;
use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::friend::apply_add_friend_record::ApplyAddFriendRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyAddFriendReq {
    pub id: i64,
    /// 申请人PeerId
    pub peer_id: String,
    /// 自己的公钥
    pub pub_key: String,
    /// 1:同意 2:拒绝
    pub status: u8,
}

impl ReplyAddFriendReq {
    pub async fn reply_add_friend(&self, _http_ctx: &HttpCtx) -> AppResult<()> {
        ApplyAddFriendRecord::update_status_by_id(self.id, self.status).await?;
        let sender = SWARM_SENDER.lock().await;
        let v = ChatEvent::Single(
            self.peer_id.to_string(),
            ChatReq::ReplyAddFriend(self.id, self.pub_key.to_string(), self.status),
        );
        let Ok(_) = sender.send(v) else {
            return Err(AppError::ChatSendError);
        };
        Ok(())
    }
}

use censor::CENSOR;
use channel::event::SWARM_SENDER;
use chat::chat::{ChatEvent, ChatReq};
use chrono::Utc;
use error::error::AppError;
use error::error::AppError::CustomError;
use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::friend::apply_add_friend_record::ApplyAddFriendRecord;
use models::friend::friends::Friends;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplyAddFriend {
    /// 被申请人PeerId
    pub peer_id: String,
    /// 昵称
    pub nickname: String,
    /// 头像
    pub avatar: String,
    /// 备注
    pub comment: String,
}

impl ApplyAddFriend {
    pub async fn apply_add_friend(&self, http_ctx: &HttpCtx) -> AppResult<()> {
        let comment = CENSOR.censor(&self.comment);
        let id = Utc::now().timestamp_nanos_opt().unwrap_or_default();
        let data = ApplyAddFriendRecord {
            id,
            comment: comment.to_owned(),
            peer_id: self.peer_id.to_string(),
            avatar: self.avatar.to_string(),
            nickname: self.nickname.to_string(),
            is_sender: 1,
            status: 0,
            is_readed: 0,
            is_deleted: 0,
        };
        data.insert().await?;
        let sender = SWARM_SENDER.lock().await;
        if let Ok(Some((nickname, avatar_url, avatar))) =
            Friends::select_nickname_avatar(&http_ctx.user.peer_id).await
        {
            let v = ChatEvent::Single(
                self.peer_id.to_string(),
                ChatReq::ApplyAddFriend(id, nickname, avatar_url, avatar, comment),
            );
            let Ok(_) = sender.send(v) else {
                return Err(AppError::ChatSendError);
            };
        } else {
            return Err(CustomError("User is not exist".to_string()));
        }
        Ok(())
    }
}

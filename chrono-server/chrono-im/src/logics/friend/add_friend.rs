use chrono::Utc;
use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::friend::friends::Friends;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendAddReq {
    pub pub_key: String,
    pub peer_id: String,
    pub avatar: String,
    pub nickname: String,
}

impl FriendAddReq {
    pub async fn friend_add(&self, _http_ctx: &HttpCtx) -> AppResult<()> {
        let id = Utc::now().timestamp_nanos_opt().unwrap_or_default();
        Friends {
            id,
            pub_key: self.pub_key.to_string(),
            peer_id: self.peer_id.to_string(),
            avatar: self.avatar.to_string(),
            nickname: self.nickname.to_string(),
            is_group: 0,
            is_deleted: 0,
        }
        .insert()
        .await?;
        Ok(())
    }
}

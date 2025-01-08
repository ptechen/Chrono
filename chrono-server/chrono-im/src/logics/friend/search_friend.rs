use channel::event::{SEARCH_FRIEND_SENDER, SWARM_SENDER, WS_SENDER};
use chat::chat::{ChatEvent, ChatReq};
use error::error::AppError;
use error::error::AppError::CustomError;
use error::result::AppResult;
use macros::if_let_err;
use middleware::auth::HttpCtx;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use swarm::keypair::generate_ed25519;
use tokio::time::sleep;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendSearchReq {
    pub peer_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendSearchRes {
    pub nickname: String,
    pub avatar: String,
}

impl FriendSearchReq {
    pub async fn friend_search(&self, http_ctx: &HttpCtx) -> AppResult<FriendSearchRes> {
        let sender = SWARM_SENDER.lock().await;
        let keypair = generate_ed25519().await?.unwrap();
        let v = ChatEvent::SearchFriend(self.peer_id.to_string(), ChatReq::SearchFriend);
        let Ok(_) = sender.send(v) else {
            return Err(AppError::ChatSendError);
        };
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<(String, String, Vec<u8>)>(1);
        {
            let mut s = SEARCH_FRIEND_SENDER.lock().await;
            *s = Some(sender);
        }
        let sleep = sleep(Duration::from_secs(3));
        tokio::pin!(sleep);
        tokio::select! {
            Some((nickname, avatar_url, avatar)) = receiver.recv() => {
                if_let_err!(tokio::fs::write(&avatar_url, avatar).await);
                Ok(FriendSearchRes{ nickname: nickname.to_string(), avatar: avatar_url.to_string() })
            }
            _ = &mut sleep => {
                let mut s = SEARCH_FRIEND_SENDER.lock().await;
                *s = None;
                Err(CustomError("No user found".to_string()))
            }
        }
    }
}

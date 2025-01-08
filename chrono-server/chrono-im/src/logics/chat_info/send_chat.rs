use censor::CENSOR;
use channel::event::SWARM_SENDER;
use chat::chat::{ChatContent, ChatEvent, ChatReq};
use chrono::Utc;
use error::error::AppError;
use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::chat_info::ChatInfo;
use serde::{Deserialize, Serialize};
use swarm::keypair::generate_ed25519;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendChatReq {
    /// 接收人
    pub receiver: String,
    /// 数据类型 1: Text 2: Img 3: File
    pub data_type: u8,
    /// 数据内容
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendChatRes {
    pub data: ChatInfo,
}

impl SendChatReq {
    pub async fn chat_send(&self, http_ctx: &HttpCtx) -> AppResult<SendChatRes> {
        let id = Utc::now().timestamp_nanos_opt().unwrap_or_default();
        let data = CENSOR.censor(&self.data);
        let data = ChatInfo {
            id,
            data_type: self.data_type,
            data: data.to_string(),
            is_sender: 1,
            status: 0,
            is_readed: 1,
            is_deleted: 0,
        };
        data.insert(&self.receiver.to_string()).await?;
        if self.receiver != http_ctx.user.peer_id {
            let sender = SWARM_SENDER.lock().await;
            let keypair = generate_ed25519().await?.unwrap();
            let sign = keypair.sign(&data.data.as_bytes())?;
            let v = ChatEvent::Single(
                self.receiver.to_string(),
                ChatReq::Chat(ChatContent::Text(id, data.data.to_string(), sign)),
            );
            let Ok(_) = sender.send(v) else {
                return Err(AppError::ChatSendError);
            };
        }
        Ok(SendChatRes { data })
    }
}

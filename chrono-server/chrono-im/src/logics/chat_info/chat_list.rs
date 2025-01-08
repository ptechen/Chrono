use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::chat_info::ChatInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatListReq {
    /// 接收人
    pub receiver: String,
    pub page_no: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatListRes {
    pub list: Vec<ChatInfo>,
}

impl ChatListReq {
    pub async fn chat_list(&self, _http_ctx: &HttpCtx) -> AppResult<ChatListRes> {
        let mut list =
            ChatInfo::select_all_by_page(&self.receiver, self.page_no, self.page_size).await?;
        list.reverse();
        Ok(ChatListRes { list })
    }
}

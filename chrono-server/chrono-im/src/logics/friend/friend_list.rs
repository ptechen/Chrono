use error::result::AppResult;
use middleware::auth::HttpCtx;
use models::friend::friends::Friends;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendListReq {
    pub page_no: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FriendListRes {
    pub list: Vec<Friends>,
}

impl FriendListReq {
    pub async fn friend_list(&self, _http_ctx: &HttpCtx) -> AppResult<FriendListRes> {
        let list = Friends::select_all_by_page(self.page_no, self.page_size).await?;
        Ok(FriendListRes { list })
    }
}

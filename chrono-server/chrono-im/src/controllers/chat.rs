use crate::logics::chat_info::chat_list::ChatListReq;
use crate::logics::chat_info::send_chat::SendChatReq;
use axum::response::IntoResponse;
use axum::Json;
use middleware::auth::{Auth, HttpCtx};
use middleware::response::AppResponse;

/// 发送聊天信息
pub async fn chat_send(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<SendChatReq>,
) -> impl IntoResponse {
    AppResponse::res(params.chat_send(&http_ctx)).await
}

/// 聊天列表
pub async fn chat_list(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<ChatListReq>,
) -> impl IntoResponse {
    AppResponse::res(params.chat_list(&http_ctx)).await
}

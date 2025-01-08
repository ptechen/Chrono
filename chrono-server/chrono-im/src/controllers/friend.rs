use crate::logics::friend::add_friend::FriendAddReq;
use crate::logics::friend::friend_list::FriendListReq;
use crate::logics::friend::reply_add_friend::ReplyAddFriendReq;
use crate::logics::friend::search_friend::FriendSearchReq;
use axum::response::IntoResponse;
use axum::Json;
use middleware::auth::{Auth, HttpCtx};
use middleware::response::AppResponse;

pub async fn friend_add(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<FriendAddReq>,
) -> impl IntoResponse {
    AppResponse::res(params.friend_add(&http_ctx)).await
}

pub async fn friend_list(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<FriendListReq>,
) -> impl IntoResponse {
    AppResponse::res(params.friend_list(&http_ctx)).await
}

pub async fn friend_search(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<FriendSearchReq>,
) -> impl IntoResponse {
    AppResponse::res(params.friend_search(&http_ctx)).await
}

pub async fn friend_reply(
    Auth(http_ctx): Auth<HttpCtx>,
    Json(params): Json<ReplyAddFriendReq>,
) -> impl IntoResponse {
    AppResponse::res(params.reply_add_friend(&http_ctx)).await
}

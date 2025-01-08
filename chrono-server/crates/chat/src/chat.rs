use error::error::AppError;
use error::result::AppResult;
use libp2p::identity::PublicKey;
use models::friend::friends::Friends;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// /// Max request size in bytes
// const REQUEST_SIZE_MAXIMUM: u64 = 1024 * 1024;
// /// Max response size in bytes
// const RESPONSE_SIZE_MAXIMUM: u64 = 10 * 1024 * 1024;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChatEvent {
    Single(String, ChatReq),
    Group(String, ChatReq),
    /// (PeerId, Key)
    SearchFriend(String, ChatReq),
    /// 回复添加好友
    RelayAddFriend(String, ChatReq),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChatReq {
    /// 申请添加好友 (id、昵称、头像url、头像、备注)
    ApplyAddFriend(i64, String, String, Vec<u8>, String),
    /// 审批添加好友 (消息内容)
    ReplyAddFriend(i64, String, u8),
    /// (接收者的peer_id，消息内容)
    Chat(ChatContent),
    SearchFriend,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChatRes {
    /// 单聊信息发送成功的响应
    SingleOk(i64),
    /// 申请添加好友发送成功
    ApplyAddFriendSendOk(i64),
    Err(String),
    /// (昵称、头像url、头像)
    SearchFriendRes(String, String, Vec<u8>),
    ReplyAddFriendRes(i64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChatContent {
    /// (文本消息, 签名信息)
    Text(i64, String, Vec<u8>),
    /// ((文件名, 格式, 数据), 签名信息)
    Files(i64, Vec<(String, String, Vec<u8>)>, Vec<u8>),
    /// ((图片名, 格式, 数据), 签名信息)
    Images(i64, Vec<(String, String, Vec<u8>)>, Vec<u8>),
    /// 同意添加
    AgreeAddFriend(i64, Friends),
}

impl Display for ChatReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApplyAddFriend(..) => f.write_str("ApplyAddFriend"),
            Self::ReplyAddFriend(..) => f.write_str("ApprovalAddFriend"),
            Self::Chat(_) => f.write_str("Chat"),
            Self::SearchFriend => f.write_str("SearchFriend"),
        }
    }
}

impl ChatReq {
    pub async fn exec_single_chat(&self, sender_peer_id: &str) -> AppResult<bool> {
        // let ok = match self {
        //     Chat::Single((_id, chat_content)) => {
        //         let public_key = Self::get_public_key(sender_peer_id).await?;
        //         let ok = match chat_content {
        //             ChatContent::Text((text, sign)) => {
        //                 public_key.verify(text.as_bytes(), sign.as_slice())
        //             }
        //             ChatContent::Files((items, sign)) => public_key.verify(
        //                 serde_json::to_vec(items).unwrap().as_slice(),
        //                 sign.as_slice(),
        //             ),
        //             ChatContent::Images((items, sign)) => public_key.verify(
        //                 serde_json::to_vec(items).unwrap().as_slice(),
        //                 sign.as_slice(),
        //             ),
        //             _ => false,
        //         };
        //         if ok {}
        //         ok
        //     }
        //     _ => false,
        // };
        return Ok(true);
    }

    pub async fn get_public_key(peer_id: &str) -> AppResult<PublicKey> {
        if let Ok(Some(friend)) = Friends::select_optional_by_peer_id(peer_id).await {
            let data: Vec<u8> = serde_json::from_str(&friend.pub_key)?;
            let public_key = PublicKey::try_decode_protobuf(data.as_slice())?;
            Ok(public_key)
        } else {
            return Err(AppError::NotFriend(peer_id.to_string()));
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

use crate::swarm::ChronoBehaviour;
use channel::event::SEARCH_FRIEND_SENDER;
use chat::chat::{ChatContent, ChatReq, ChatRes};
use error::result::AppResult;
use libp2p::request_response::{Event, Message};
use libp2p::{request_response, PeerId, Swarm};
use macros::if_let_err;
use models::chat_info::ChatInfo;
use models::friend::apply_add_friend_record::ApplyAddFriendRecord;
use models::friend::friends::Friends;

pub async fn request_response_event(
    swarm: &mut Swarm<ChronoBehaviour>,
    event: request_response::Event<ChatReq, ChatRes>,
    local_peer_id: PeerId,
) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        Event::Message { peer, message } => match message {
            Message::Request {
                request_id,
                request,
                channel,
            } => match request {
                ChatReq::ReplyAddFriend(id, pub_key, status) => {
                    ApplyAddFriendRecord::update_status_by_id(id, status).await?;
                    if let Ok(Some(data)) = ApplyAddFriendRecord::select_optional_by_id(id).await {
                        if_let_err!(
                            Friends {
                                id,
                                pub_key,
                                peer_id: peer.to_string(),
                                avatar: data.avatar.to_string(),
                                nickname: data.nickname.to_string(),
                                is_group: 0,
                                is_deleted: 0,
                            }
                            .insert()
                            .await
                        );
                        if_let_err!(swarm
                            .behaviour_mut()
                            .request_response
                            .send_response(channel, ChatRes::ReplyAddFriendRes(id)));
                    }
                }
                ChatReq::Chat(chat_content) => match chat_content {
                    ChatContent::Text(id, data, sign) => {
                        if_let_err!(
                            ChatInfo {
                                id,
                                data_type: 1,
                                data,
                                is_sender: 0,
                                status: 2,
                                is_readed: 0,
                                is_deleted: 0,
                            }
                            .insert(&peer.to_string())
                            .await
                        );
                        if_let_err!(swarm
                            .behaviour_mut()
                            .request_response
                            .send_response(channel, ChatRes::SingleOk(id)));
                    }
                    ChatContent::Files(id, content, sign) => {}
                    ChatContent::Images(id, content, sign) => {}
                    ChatContent::AgreeAddFriend(_, _) => {}
                },
                ChatReq::SearchFriend => {
                    if let Ok(Some((nickname, avatar_url, avatar))) =
                        Friends::select_nickname_avatar(&local_peer_id.to_string()).await
                    {
                        swarm
                            .behaviour_mut()
                            .request_response
                            .send_response(
                                channel,
                                ChatRes::SearchFriendRes(nickname, avatar_url, avatar),
                            )
                            .unwrap_or_default();
                    }
                }
                ChatReq::ApplyAddFriend(id, nickname, avatar_url, avatar, comment) => {
                    if_let_err!(tokio::fs::write(&avatar_url, avatar).await);
                    if_let_err!(
                        ApplyAddFriendRecord {
                            id,
                            peer_id: peer.to_string(),
                            avatar: avatar_url.to_string(),
                            nickname,
                            comment,
                            is_sender: 0,
                            status: 0,
                            is_readed: 0,
                            is_deleted: 0,
                        }
                        .insert()
                        .await
                    );
                    if_let_err!(swarm
                        .behaviour_mut()
                        .request_response
                        .send_response(channel, ChatRes::ApplyAddFriendSendOk(id)));
                }
            },
            Message::Response {
                request_id,
                response,
            } => match response {
                ChatRes::SingleOk(_) => {}
                ChatRes::Err(_) => {}
                ChatRes::ApplyAddFriendSendOk(_) => {}
                ChatRes::SearchFriendRes(nickname, avatar_url, avatar) => {
                    let sender = SEARCH_FRIEND_SENDER.lock().await;
                    if sender.is_some() {
                        let sender = sender.clone().unwrap();
                        if_let_err!(sender.send((nickname, avatar_url, avatar)).await);
                    }
                }
                ChatRes::ReplyAddFriendRes(_) => {}
            },
        },
        Event::OutboundFailure { .. } => {}
        Event::InboundFailure { .. } => {}
        Event::ResponseSent { .. } => {}
    }
    Ok(())
}

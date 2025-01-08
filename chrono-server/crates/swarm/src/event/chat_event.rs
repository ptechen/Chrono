use crate::swarm::ChronoBehaviour;
use chat::chat::ChatEvent;
use error::result::AppResult;
use libp2p::gossipsub::IdentTopic;
use libp2p::{PeerId, Swarm};
use macros::if_let_err;
use std::str::FromStr;

pub fn chat_event(swarm: &mut Swarm<ChronoBehaviour>, event: ChatEvent) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        ChatEvent::Single(receiver, chat) => {
            let peer_id = PeerId::from_str(&receiver)?;
            swarm
                .behaviour_mut()
                .request_response
                .send_request(&peer_id, chat);
        }
        ChatEvent::Group(topic, chat) => {
            if_let_err!(swarm
                .behaviour_mut()
                .gossipsub
                .publish(IdentTopic::new(topic), chat.to_vec()));
        }
        ChatEvent::SearchFriend(_, _) => {}
        ChatEvent::RelayAddFriend(_, _) => {}
    }
    Ok(())
}

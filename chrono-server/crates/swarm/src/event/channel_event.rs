use crate::swarm::ChronoBehaviour;
use chat::chat::ChatEvent;
use error::result::AppResult;
use libp2p::Swarm;

pub fn channel_event(swarm: &mut Swarm<ChronoBehaviour>, event: ChatEvent) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        ChatEvent::Single(_, _) => {}
        ChatEvent::Group(_, _) => {}
        ChatEvent::SearchFriend(peer_id, data) => {
            let peer_id = peer_id.parse()?;
            swarm
                .behaviour_mut()
                .request_response
                .send_request(&peer_id, data);
        }
        ChatEvent::RelayAddFriend(peer_id, data) => {
            let peer_id = peer_id.parse()?;
            swarm
                .behaviour_mut()
                .request_response
                .send_request(&peer_id, data);
        }
    }
    Ok(())
}

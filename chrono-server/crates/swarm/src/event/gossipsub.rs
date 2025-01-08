use crate::swarm::ChronoBehaviour;
use chat::chat::{ChatContent, ChatReq};
use error::result::AppResult;
use libp2p::gossipsub::Event;
use libp2p::{gossipsub, Swarm};

pub async fn gossipsub_event(
    swarm: &mut Swarm<ChronoBehaviour>,
    event: gossipsub::Event,
) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        Event::Message {
            propagation_source,
            message_id,
            message,
        } => {}
        Event::Subscribed { .. } => {}
        Event::Unsubscribed { .. } => {}
        Event::GossipsubNotSupported { .. } => {}
    }
    Ok(())
}

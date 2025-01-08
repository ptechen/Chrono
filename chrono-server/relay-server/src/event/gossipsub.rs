use crate::ChronoBehaviour;
use error::result::AppResult;
use libp2p::gossipsub::Event;
use libp2p::Swarm;

pub fn gossipsub_event(swarm: &mut Swarm<ChronoBehaviour>, event: Event) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        Event::Message {
            propagation_source,
            message_id,
            message,
        } => {}
        Event::Subscribed { peer_id, topic } => {}
        Event::Unsubscribed { peer_id, topic } => {}
        Event::GossipsubNotSupported { peer_id } => {}
    }
    Ok(())
}

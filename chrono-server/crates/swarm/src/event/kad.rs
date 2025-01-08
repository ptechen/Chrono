use crate::swarm::ChronoBehaviour;
use error::result::AppResult;
use libp2p::{kad, Swarm};

pub fn kad_event(swarm: &mut Swarm<ChronoBehaviour>, event: kad::Event) -> AppResult<()> {
    tracing::info!("{:?}", event);
    Ok(())
}

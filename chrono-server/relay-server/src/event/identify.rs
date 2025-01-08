use error::result::AppResult;
use libp2p::identify::Event;
use libp2p::swarm::Swarm;
use libp2p::Multiaddr;

use crate::ChronoBehaviour;

pub fn identify_event(swarm: &mut Swarm<ChronoBehaviour>, event: Event) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        Event::Received { peer_id, info } => {
            let mut list = vec![];
            for address in info.listen_addrs {
                let address = address.to_string();
                if !address.contains("127.0.0.1")
                    && !address.contains("p2p")
                    && address.contains("ip4")
                {
                    list.push(address);
                }
            }
            for address in &list {
                let address: Multiaddr = address.parse()?;
                swarm.add_peer_address(peer_id, address.clone());
                let routing_update = swarm.behaviour_mut().kad.add_address(&peer_id, address);
                tracing::info!("{:?}", routing_update);
            }
        }
        _ => {} //        Event::Sent { peer_id } => {}
                //        Event::Pushed { peer_id, info } => {}
                //        Event::Error { peer_id, error } => {}
    }
    Ok(())
}

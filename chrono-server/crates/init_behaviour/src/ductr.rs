use libp2p::{dcutr, PeerId};

pub fn init_dcutr(local_peer_id: PeerId) -> dcutr::Behaviour {
    dcutr::Behaviour::new(local_peer_id)
}

use libp2p::{autonat, PeerId};

pub fn init_autonat(local_peer_id: PeerId) -> autonat::Behaviour {
    autonat::Behaviour::new(local_peer_id, autonat::Config::default())
}

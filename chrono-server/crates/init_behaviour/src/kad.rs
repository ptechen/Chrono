use libp2p::kad::store::MemoryStore;
use libp2p::{kad, PeerId, StreamProtocol};
use std::time::Duration;

const KAD_PROTO_NAME: StreamProtocol = StreamProtocol::new("/chrono/kad/1.0.0");

pub fn init_kad(local_peer_id: PeerId) -> kad::Behaviour<MemoryStore> {
    let mut cfg = kad::Config::default();
    cfg.set_protocol_names(vec![KAD_PROTO_NAME]);
    cfg.set_query_timeout(Duration::from_secs(5 * 60));
    let store = MemoryStore::new(local_peer_id);
    kad::Behaviour::with_config(local_peer_id, store, cfg)
}

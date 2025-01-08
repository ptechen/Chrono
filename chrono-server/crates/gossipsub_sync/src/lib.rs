use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GossipSubSync {
    /// 同步（PeerId, Multiaddr）
    SyncPeerMultiaddr((String, Vec<String>)),
}

impl GossipSubSync {
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

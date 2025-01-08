use std::str::FromStr;

use error::result::AppResult;
use libp2p::kad::{GetRecordOk, PeerRecord, QueryResult, Record};
use libp2p::{kad, Multiaddr, Swarm};

use crate::ChronoBehaviour;

pub fn kad_event(swarm: &mut Swarm<ChronoBehaviour>, event: kad::Event) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        kad::Event::InboundRequest { request } => {
            inbound_request(swarm, request)?;
        }
        kad::Event::OutboundQueryProgressed {
            id,
            result,
            stats,
            step,
        } => match result {
            QueryResult::Bootstrap(_) => {}
            QueryResult::GetClosestPeers(data) => {
                let data = data.unwrap();
                let val = String::from_utf8_lossy(&data.key).to_string();
                tracing::info!("GetClosestPeers: {:?} {}", data.peers, val);
            }
            QueryResult::GetProviders(_) => {}
            QueryResult::StartProviding(_) => {}
            QueryResult::RepublishProvider(_) => {}
            QueryResult::GetRecord(Ok(data)) => match data {
                GetRecordOk::FoundRecord(PeerRecord {
                    peer,
                    record:
                        Record {
                            key,
                            value,
                            publisher,
                            expires,
                        },
                }) => {
                    let peer_id = String::from_utf8_lossy(&key.to_vec()).to_string();
                    let peer_id = peer_id.parse()?;
                    let addresses: Vec<String> = serde_json::from_slice(&value)?;
                    for address in addresses {
                        swarm.add_peer_address(peer_id, Multiaddr::from_str(&address)?);
                    }
                }
                GetRecordOk::FinishedWithNoAdditionalRecord { cache_candidates } => todo!(),
            },
            QueryResult::GetRecord(Err(e)) => {
                // tracing::error!("QueryResult::GetRecord: {e}");
            }
            QueryResult::PutRecord(_) => {}
            QueryResult::RepublishRecord(_) => {}
        },
        kad::Event::RoutingUpdated {
            peer,
            is_new_peer,
            addresses,
            bucket_range,
            old_peer,
        } => {}
        kad::Event::UnroutablePeer { peer } => {}
        kad::Event::RoutablePeer { peer, address } => {}
        kad::Event::PendingRoutablePeer { peer, address } => {}
        kad::Event::ModeChanged { new_mode } => {}
    }
    Ok(())
}

fn inbound_request(
    _swarm: &mut Swarm<ChronoBehaviour>,
    request: kad::InboundRequest,
) -> AppResult<()> {
    match request {
        kad::InboundRequest::FindNode { num_closer_peers } => {}
        kad::InboundRequest::GetProvider {
            num_closer_peers,
            num_provider_peers,
        } => {}
        kad::InboundRequest::AddProvider { record } => {}
        kad::InboundRequest::GetRecord {
            num_closer_peers,
            present_locally,
        } => {}
        kad::InboundRequest::PutRecord {
            source,
            connection,
            record,
        } => {}
    }
    Ok(())
}

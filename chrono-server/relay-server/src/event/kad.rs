use error::result::AppResult;
use libp2p::{kad, Swarm};

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
        } => {}
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
    swarm: &mut Swarm<ChronoBehaviour>,
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

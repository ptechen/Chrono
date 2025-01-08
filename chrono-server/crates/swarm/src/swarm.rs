use crate::event::channel_event::channel_event;
use crate::event::gossipsub::gossipsub_event;
use crate::event::kad::kad_event;
use crate::event::request_response::request_response_event;
use channel::event::SWARM_RECEIVER;
use chat::chat_codec::{ChatCodeC, CHAT_PROTOCOL_NAME};
use error::result::AppResult;
use futures::StreamExt;
use init_behaviour::autonat::init_autonat;
use init_behaviour::gossipsub::init_gossipsub;
use init_behaviour::identify::init_identify;
use init_behaviour::kad::init_kad;
use init_behaviour::request_response::init_request_response;
use libp2p::gossipsub::IdentTopic;
use libp2p::identity::Keypair;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Mode;
use libp2p::{
    autonat, dcutr, gossipsub, identify, kad, noise, ping, relay, request_response,
    swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId, Swarm,
};
use macros::if_let_err;
use middleware::ticker::SERVER_CLOSE_TAG;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::select;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct ChronoBehaviour {
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
    pub dcutr: dcutr::Behaviour,
    pub relay_client: relay::client::Behaviour,
    pub autonat: autonat::Behaviour,
    pub request_response: request_response::Behaviour<ChatCodeC>,
    // pub // mdns: mdns::tokio::Behaviour,
    pub kad: kad::Behaviour<MemoryStore>,
    pub gossipsub: gossipsub::Behaviour,
}

fn init_swarm(keypair: &Keypair) -> AppResult<Swarm<ChronoBehaviour>> {
    let local_peer_id = keypair.public().to_peer_id();
    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_dns()
        .unwrap()
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|keypair, relay_client| {
            let gossipsub = init_gossipsub(keypair.clone())?;
            let identify = init_identify(keypair.public());
            let dcutr = dcutr::Behaviour::new(local_peer_id);
            let autonat = init_autonat(local_peer_id);
            let request_response = init_request_response();
            let kad = init_kad(local_peer_id);
            let ping = ping::Behaviour::default();
            Ok(ChronoBehaviour {
                ping,
                identify,
                dcutr,
                relay_client,
                autonat,
                request_response,
                gossipsub,
                kad,
            })
        })
        .unwrap()
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}

pub async fn swarm(keypair: Keypair) -> AppResult<()> {
    let local_peer_id = keypair.public().to_peer_id();
    let mut swarm = init_swarm(&keypair)?;
    swarm.behaviour_mut().kad.set_mode(Some(Mode::Client));
    let peer_id = PeerId::from_str("12D3KooWMbRvqThz9uEPDo4uXiW8rz242XNUQVV7ksDRYXNtZZwh").unwrap();
    let address = Multiaddr::from_str("/ip4/127.0.0.1/tcp/53595").unwrap();
    tracing::info!(
        "{:?}",
        swarm.behaviour_mut().kad.add_address(&peer_id, address)
    );
    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("chrono-sync");
    // subscribes to our topic
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    tracing::info!(
        "Enter messages via STDIN and they will be sent to connected peers using Gossipsub"
    );
    let mut receiver = SWARM_RECEIVER.lock().await;
    // Kick it off
    while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
        select! {
            Some(event) = receiver.recv() => {
                if_let_err!(channel_event(&mut swarm, event));
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Identify(event)) => {
                    tracing::info!("{:?}", event);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Ping(event)) => {
                    tracing::info!("{:?}", event);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Autonat(event)) => {
                    tracing::info!("{:?}", event);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Dcutr(event)) => {
                    tracing::info!("{:?}", event);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Gossipsub(event)) => {
                    if_let_err!(gossipsub_event(&mut swarm, event).await);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::RequestResponse(event)) => {
                    if_let_err!(request_response_event(&mut swarm, event, local_peer_id).await);
                },
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Kad(event)) => {
                    if_let_err!(kad_event(&mut swarm, event));
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!("Local node is listening on {address}");
                }
                _  => {}
            }
        }
    }
    Ok(())
}

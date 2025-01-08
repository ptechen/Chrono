pub mod event;
use chat::chat_codec::ChatCodeC;
use error::result::AppResult;
use futures::StreamExt;
use init_behaviour::autonat::init_autonat;
use init_behaviour::ductr::init_dcutr;
use init_behaviour::gossipsub::init_gossipsub;
use init_behaviour::identify::init_identify;
use init_behaviour::kad::init_kad;
use init_behaviour::ping::init_ping;
use init_behaviour::request_response::init_request_response;
use libp2p::identity::Keypair;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Mode;
use libp2p::multiaddr::Protocol;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{
    autonat, dcutr, gossipsub, identify, kad, noise, ping, relay, request_response, tcp, yamux,
    Multiaddr, Swarm,
};
use macros::if_let_err;
use middleware::ticker::SERVER_CLOSE_TAG;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::select;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::FmtSubscriber;

use crate::event::gossipsub::gossipsub_event;
use crate::event::identify::identify_event;
use crate::event::kad::kad_event;
use crate::event::req_res::request_response_event;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct ChronoBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    autonat: autonat::Behaviour,
    dcutr: dcutr::Behaviour,
    relay: relay::Behaviour,
    gossipsub: gossipsub::Behaviour,
    request_response: request_response::Behaviour<ChatCodeC>,
    kad: kad::Behaviour<MemoryStore>,
}

fn init_swarm(keypair: &Keypair) -> AppResult<Swarm<ChronoBehaviour>> {
    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|keypair| {
            let local_peer_id = keypair.public().to_peer_id();
            // identify::Behaviour
            let identify = init_identify(keypair.public()); // ping::Behaviour
            let ping = init_ping();
            // autonat::Behaviour
            let autonat = init_autonat(local_peer_id);
            // dcutr::Behaviour
            let dcutr = init_dcutr(local_peer_id);
            let relay = relay::Behaviour::new(local_peer_id, Default::default());
            // build a gossipsub network behaviour
            let gossipsub = init_gossipsub(keypair.clone())?;
            let request_response = init_request_response();
            let kad = init_kad(local_peer_id);
            Ok(ChronoBehaviour {
                ping,
                identify,
                autonat,
                dcutr,
                relay,
                gossipsub,
                request_response,
                kad,
            })
        })
        .unwrap()
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}

pub async fn swarm(keypair: Keypair) -> AppResult<()> {
    let mut swarm = init_swarm(&keypair)?;
    swarm.behaviour_mut().kad.set_mode(Some(Mode::Server));

    let _ = swarm.behaviour_mut().kad.bootstrap();
    // // Create a Gossipsub topic
    // let topic = gossipsub::IdentTopic::new("chrono-sync");
    // // subscribes to our topic
    // swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    let listen_addr_tcp = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Tcp(65001));
    swarm.listen_on(listen_addr_tcp)?;

    let listen_addr_quic = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Udp(65001))
        .with(Protocol::QuicV1);
    swarm.listen_on(listen_addr_quic)?;
    // Kick it off
    while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Identify(event)) => {
                    if_let_err!(identify_event(&mut swarm, event));
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
                SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!("Local node is listening on {address}");
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Gossipsub(event)) => {
                    if_let_err!(gossipsub_event(&mut swarm, event), "ChronoBehaviourEvent::Gossipsub");
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::RequestResponse(event)) => {
                    if_let_err!(request_response_event(&mut swarm, event),"ChronoBehaviourEvent::RequestResponse");
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Kad(event)) => {
                    if_let_err!(kad_event(&mut swarm, event), "ChronoBehaviourEvent::Kad");
                }
                _  => {}
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_str("info").unwrap())
        .finish()
        .init();

    let keypair = generate_ed25519(0);
    let _ = swarm(keypair).await;
    Ok(())
}

fn generate_ed25519(secret_key_seed: u8) -> Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;

    Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}

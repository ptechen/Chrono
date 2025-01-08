use chat::chat::{ChatContent, ChatReq, ChatRes};
use chat::chat_codec::ChatCodeC;
use futures::stream::StreamExt;
use gossipsub_sync::GossipSubSync;
use init_behaviour::autonat::init_autonat;
use init_behaviour::gossipsub::init_gossipsub;
use init_behaviour::identify::init_identify;
use init_behaviour::kad::init_kad;
use init_behaviour::request_response::init_request_response;
use libp2p::identity::Keypair;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Mode;
use libp2p::multiaddr::Protocol;
use libp2p::{
    autonat, dcutr, gossipsub, identify, kad, noise, relay, request_response,
    swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId,
};
use macros::if_let_err;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::FmtSubscriber;

pub mod event;
use crate::event::kad::kad_event;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct ChronoBehaviour {
    identify: identify::Behaviour,
    dcutr: dcutr::Behaviour,
    relay_client: relay::client::Behaviour,
    autonat: autonat::Behaviour,
    request_response: request_response::Behaviour<ChatCodeC>,
    gossipsub: gossipsub::Behaviour,
    kad: kad::Behaviour<MemoryStore>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_str("info").unwrap())
        .finish()
        .init();
    let keypair = Keypair::generate_ed25519();
    let local_peer_id = keypair.public().to_peer_id();
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_dns()?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|keypair, relay_client| {
            let gossipsub = init_gossipsub(keypair.clone())?;
            let identify = init_identify(keypair.public());
            let dcutr = dcutr::Behaviour::new(local_peer_id);
            let autonat = init_autonat(local_peer_id);
            let request_response = init_request_response();
            let kad = init_kad(local_peer_id);
            Ok(ChronoBehaviour {
                identify,
                dcutr,
                relay_client,
                autonat,
                request_response,
                gossipsub,
                kad,
            })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    // // Create a Gossipsub topic
    // let topic = gossipsub::IdentTopic::new("chrono-sync");
    // // subscribes to our topic
    // swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    let relay_peer_id = PeerId::from_str("12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN")?;
    let relay_address = Multiaddr::from_str("/ip4/10.37.129.2/udp/65001/quic-v1")?;
    swarm
        .behaviour_mut()
        .kad
        .add_address(&relay_peer_id, relay_address.clone());
    if_let_err!(swarm.behaviour_mut().kad.bootstrap());
    // swarm.behaviour_mut().gossipsub.add_explicit_peer(&PeerId::from_str("12D3KooWM2u3TpAXwZSy7pA72ArDfmRAfeQ6kx1Tp3ZdrUeeVk7R").unwrap());
    swarm.behaviour_mut().kad.set_mode(Some(Mode::Client));
    // let peer_id = PeerId::from_str("12D3KooWN3YB1jZXtyc91joA4ccQ72xSfKCKWYyZt8Qan2pBL5kW").unwrap();
    // swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
    // println!("{:?}",swarm.behaviour_mut().kad.add_address(&peer_id, Multiaddr::from_str("/ip4/127.0.0.1/tcp/9000").unwrap()));
    // swarm
    //     .listen_on(Multiaddr::from_str("/ip4/192.168.1.148/tcp/65001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN").unwrap().with(Protocol::P2pCircuit))
    //     .unwrap();
    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm
        .behaviour_mut()
        .autonat
        .add_server(relay_peer_id.clone(), Some(relay_address.clone()));
    swarm.add_peer_address(relay_peer_id, relay_address.clone());
    swarm
        .listen_on(
            Multiaddr::from_str("/ip4/10.37.129.2/udp/65001/quic-v1")
                .unwrap()
                .with_p2p(
                    PeerId::from_str("12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN")
                        .unwrap(),
                )
                .unwrap()
                .with(Protocol::P2pCircuit),
        )
        .unwrap();
    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                // let data:Chat = serde_json::from_str(&line).unwrap();
                let data:Vec<&str> = line.split("$").collect();
                let peer_id = PeerId::from_str(data.get(0).unwrap()).unwrap();
                let data:&str = data.get(1).unwrap();
                // let q = swarm.behaviour_mut().kad.get_closest_peers(peer_id);
                // swarm.behaviour_mut()
                swarm.behaviour_mut().request_response.send_request(&peer_id, ChatReq::Chat(ChatContent::Text(0,data.to_string(), vec![])));
                // swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.as_bytes().to_vec());
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Dcutr(event)) => {
                    tracing::info!(?event);
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => {
                    tracing::info!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    );
                },

                SwarmEvent::NewListenAddr { address,..} => {
                    tracing::info!("Local node is listening on {address}");
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::Kad(event)) => {
                    if_let_err!(kad_event(&mut swarm, event));
                }
                SwarmEvent::Behaviour(ChronoBehaviourEvent::RequestResponse(request_response::Event::Message {peer,message})) => {
                    tracing::info!("{peer}, message: {:?}", message);
                    match message {
                        request_response::Message::Request {request_id,request,channel} => {
                            tracing::info!("Request: {:?}", request);
                            // if let Err(e) = swarm.behaviour_mut().request_response.send_response(channel, ChatRes::SingleOk(request)) {
                            //     tracing::error!("send_response: {:?}", e);
                            // }
                            // tracing::info!("Request: OK");
                        }
                        request_response::Message::Response {..} => {

                        }
                    }
                }
                _ => {}
            }
        }
    }
}

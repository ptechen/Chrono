use libp2p::ping;

pub fn init_ping() -> ping::Behaviour {
    ping::Behaviour::new(ping::Config::new())
}

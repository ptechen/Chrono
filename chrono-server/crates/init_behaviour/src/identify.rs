use libp2p::identity::PublicKey;

pub fn init_identify(public: PublicKey) -> libp2p::identify::Behaviour {
    libp2p::identify::Behaviour::new(libp2p::identify::Config::new(
        "/chrono/identify/0.0.1".to_string(),
        public,
    ))
}

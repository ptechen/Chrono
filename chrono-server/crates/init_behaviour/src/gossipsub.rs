use error::result::AppResult;
use libp2p::{gossipsub, identity::Keypair};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};
use tokio::io;

pub fn init_gossipsub(keypair: Keypair) -> AppResult<gossipsub::Behaviour> {
    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

    // build a gossipsub network behaviour
    Ok(gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair),
        gossipsub_config,
    )
    .unwrap())
}

use chat::chat_codec::{ChatCodeC, CHAT_PROTOCOL_NAME};
use libp2p::request_response;

pub fn init_request_response() -> request_response::Behaviour<ChatCodeC> {
    request_response::Behaviour::<ChatCodeC>::new(
        vec![(CHAT_PROTOCOL_NAME, request_response::ProtocolSupport::Full)],
        request_response::Config::default(),
    )
}

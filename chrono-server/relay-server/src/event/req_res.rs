use crate::ChronoBehaviour;
use chat::chat::{ChatReq, ChatRes};
use error::result::AppResult;
use libp2p::request_response::{Event, Message};
use libp2p::Swarm;
use macros::if_let_err;

pub fn request_response_event(
    swarm: &mut Swarm<ChronoBehaviour>,
    event: Event<ChatReq, ChatRes>,
) -> AppResult<()> {
    tracing::info!("{:?}", event);
    match event {
        Event::Message { peer, message } => match message {
            Message::Request {
                request_id,
                request,
                channel,
            } => {}
            Message::Response {
                request_id,
                response,
            } => {}
        },
        Event::OutboundFailure {
            peer,
            request_id,
            error,
        } => {}
        Event::InboundFailure {
            peer,
            request_id,
            error,
        } => {}
        Event::ResponseSent { peer, request_id } => {}
    }
    Ok(())
}

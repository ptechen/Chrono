use crate::logics::ws::handle_socket;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct QueryParams {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    tracing::info!("{}", "test");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

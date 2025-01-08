use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use middleware::shutdown_signal::shutdown_signal;
use middleware::ticker::SERVER_CLOSE_TAG;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;

/// Actual websocket statemachine (one will be spawned per connection)
pub async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    // 通过拆分套接字，我们可以同时发送和接收。在此示例中，我们将发送
    // 基于某种服务器的内部事件（即 .timer）向客户端发送的未经请求的消息。
    let (mut sender, mut receiver) = socket.split();
    while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
        tokio::select! {
            Some(Ok(msg)) = receiver.next() => {
                match msg {
                    Message::Text(_data) => {
                        // tracing::info!("{data}");
                        // let _ = SWARM_SENDER.lock().await.send(Event::Chat(Ch));
                    }
                    Message::Binary(_data) => {
                        // let data = serde_json::from_slice(data.as_slice()).unwrap()
                    }
                    Message::Ping(data) => {
                        if let Err(e) = sender.send(Message::Pong(data)).await {
                            tracing::error!("{e}");
                        }
                    }
                    Message::Pong(data) => {
                        tracing::debug!("pong: {:?}", data);
                    }
                    Message::Close(_) => {
                        return
                    }
                }
            },
            _ = shutdown_signal() => {
                return
            }
        }
    }
    tracing::debug!("Websocket context {who} destroyed");
}

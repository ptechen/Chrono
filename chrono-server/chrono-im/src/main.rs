mod controllers;
pub mod logics;

use crate::controllers::app;
use channel::event::{SWARM_RECEIVER, SWARM_SENDER};
use chat::chat::{ChatEvent, ChatReq};
use error::result::AppResult;
use middleware::shutdown_signal::shutdown_signal;
use middleware::ticker::SERVER_CLOSE_TAG;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use swarm::keypair::generate_ed25519;
use swarm::swarm::swarm;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> AppResult<()> {
    FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_str("debug").unwrap())
        .finish()
        .init();
    {
        let (s, r) = tokio::sync::mpsc::unbounded_channel::<ChatEvent>();
        let mut sender = SWARM_SENDER.lock().await;
        *sender = s;
        let mut receiver = SWARM_RECEIVER.lock().await;
        *receiver = r;
    }
    // tokio::spawn(async move {
    //     if let Ok(Some(keypair)) = generate_ed25519().await {
    //         let _ = swarm(keypair).await;
    //     }
    // });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:65000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    SERVER_CLOSE_TAG.store(false, Ordering::Relaxed);
    Ok(())
}

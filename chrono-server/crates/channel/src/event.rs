use chat::chat::ChatEvent;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub static SWARM_RECEIVER: Lazy<Mutex<UnboundedReceiver<ChatEvent>>> = Lazy::new(|| {
    let (_, r) = tokio::sync::mpsc::unbounded_channel::<ChatEvent>();
    Mutex::new(r)
});

pub static SWARM_SENDER: Lazy<Mutex<UnboundedSender<ChatEvent>>> = Lazy::new(|| {
    let (s, _) = tokio::sync::mpsc::unbounded_channel::<ChatEvent>();
    Mutex::new(s)
});

pub static WS_RECEIVER: Lazy<Mutex<UnboundedReceiver<ChatEvent>>> = Lazy::new(|| {
    let (_, r) = tokio::sync::mpsc::unbounded_channel::<ChatEvent>();
    Mutex::new(r)
});

pub static WS_SENDER: Lazy<Mutex<UnboundedSender<ChatEvent>>> = Lazy::new(|| {
    let (s, _) = tokio::sync::mpsc::unbounded_channel::<ChatEvent>();
    Mutex::new(s)
});

pub static SEARCH_FRIEND_SENDER: Lazy<Mutex<Option<Sender<(String, String, Vec<u8>)>>>> =
    Lazy::new(|| Mutex::new(None));

use error::result::AppResult;
use libp2p::identity::Keypair;
use middleware::ticker::SERVER_CLOSE_TAG;
use models::owner_info::OwnerInfo;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::time::sleep;

pub async fn generate_ed25519() -> AppResult<Option<Keypair>> {
    while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
        if let Ok(Some(owner_info)) = OwnerInfo::select_optional().await {
            return Ok(generate_ed25519_by_phrase(&owner_info.phrase).await.ok());
        }
        sleep(Duration::from_secs(3)).await;
    }
    Ok(None)
}

pub async fn generate_ed25519_by_phrase(phrase: &str) -> AppResult<Keypair> {
    let data = format!("{}", phrase).replace(" ", "");
    let items = data.as_bytes();
    let mut bytes = [0u8; 32];
    for (idx, val) in items.iter().enumerate() {
        if idx >= 32 {
            break;
        }
        bytes[idx] = *val;
    }
    return Ok(Keypair::ed25519_from_bytes(bytes).unwrap());
}

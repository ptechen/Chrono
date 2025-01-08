use avatar::avatar;
use bip39::{Language, Mnemonic, MnemonicType};
use chrono::Utc;
use crc64fast::Digest;
use error::result::AppResult;
use models::avatars::Avatars;
use models::friend::friends::Friends;
use models::owner_info::OwnerInfo;
use nickname::generate_nickname;
use serde::{Deserialize, Serialize};
use swarm::keypair::generate_ed25519_by_phrase;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignupReq {
    pub email: String,
    pub code: String,
    pub password: String,
}

impl SignupReq {
    pub async fn signup(&self) -> AppResult<()> {
        let mn = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mn.phrase();
        if let Ok(keypair) = generate_ed25519_by_phrase(phrase).await {
            let peer_id = keypair.public().to_peer_id().to_base58();
            tracing::info!("{}", peer_id);
            let nickname = generate_nickname(&peer_id);
            let avatar_webp = avatar(&peer_id);
            let mut c = Digest::new();
            c.write(peer_id.as_bytes());
            let checksum = c.sum64();
            let avatar = format!("avatar/{}.webp", checksum);
            Avatars {
                id: checksum.to_string(),
                avatar: avatar_webp,
            }
            .update_insert()
            .await?;
            let id = Utc::now().timestamp_nanos_opt().unwrap_or_default();
            Friends {
                id,
                pub_key: serde_json::to_string(&keypair.public().encode_protobuf())?,
                peer_id,
                avatar,
                nickname,
                is_group: 0,
                is_deleted: 0,
            }
            .insert()
            .await?;
        }
        let time = Utc::now().timestamp();
        OwnerInfo {
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            phrase: phrase.to_string(),
            is_deleted: 0,
            updated_at: time,
            created_at: time,
        }
        .insert()
        .await?;
        Ok(())
    }
}

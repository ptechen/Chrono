use chrono::Utc;
use crc64fast::Digest;
use error::error::{AppError, ErrTrait};
use error::result::AppResult;
use libp2p::identity::Keypair;
use middleware::user::User;
use models::owner_info::OwnerInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginRes {
    pub token: String,
}

impl LoginReq {
    pub async fn login(&self) -> AppResult<LoginRes> {
        return if let Some(owner_info) =
            OwnerInfo::select_optional_by_email_password(&self.email, &self.password).await?
        {
            let mut bytes = [0u8; 32];
            for (idx, val) in owner_info.phrase.as_bytes().iter().enumerate() {
                if idx >= 32 {
                    break;
                }
                bytes[idx] = *val;
            }
            let keypair = Keypair::ed25519_from_bytes(&mut bytes)?;
            let peer_id = keypair.public().to_peer_id().to_base58();
            let mut c = Digest::new();
            c.write(peer_id.as_bytes());
            let checksum = c.sum64();
            let token = User {
                peer_id,
                id: checksum,
                exp: (Utc::now().timestamp() + 7 * 60 * 60 * 24) as usize,
            }
            .encode()
            .await?;
            Ok(LoginRes { token })
        } else {
            Err(AppError::UserIsNotExist.to_err())
        };
    }
}

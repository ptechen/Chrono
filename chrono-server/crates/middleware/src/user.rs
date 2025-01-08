use error::result::AppResult;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct User {
    pub peer_id: String,
    pub id: u64,
    pub exp: usize,
}

impl User {
    pub async fn encode(&self) -> AppResult<String> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("chrono".as_ref()),
        )?;
        Ok(token)
    }

    pub async fn decode(token: &str) -> AppResult<Self> {
        let token = decode::<Self>(
            &token,
            &DecodingKey::from_secret("chrono".as_ref()),
            &Validation::default(),
        )?;
        Ok(token.claims)
    }
}

#[tokio::test]
async fn test() {
    let token = User {
        peer_id: "".to_string(),
        id: 0,
        exp: Utc::now().timestamp() as usize + 7 * 60 * 60 * 24,
    }
    .encode()
    .await
    .unwrap();
    // let data = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwZWVyX2lkIjoiMTJEM0tvb1dGRUdqcjZmTTJHVVdHNnJGYVNtTEpYU0dOVFU2VHRUV3k2blM3ZnBCVUxmMSIsImlkIjoxODA4Njg3NTc2NDA0MjUyNDE4OSwiZXhwaXJlZCI6MTcxODc1NTQ4MX0.rA4bQY2HQx1zfkF9VpgQeOtJWKrvKPrKIyMUcZucFy0";
    if let Err(e) = User::decode(&token).await {
        println!("{e}");
    }
}

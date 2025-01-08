use error::result::AppResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HelloWorldReq {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HelloWorldRes {
    pub data: String,
}

impl HelloWorldReq {
    pub async fn hello_world(&self) -> AppResult<HelloWorldRes> {
        Ok(HelloWorldRes {
            data: String::from("Hello World"),
        })
    }
}

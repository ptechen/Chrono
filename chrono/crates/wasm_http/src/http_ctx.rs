use configs::CHRONO_IM_URL;
use error::error::AppError::CustomError;
use error::result::AppResult;
use gloo_net::http::{Headers, Request};
use leptos::logging::log;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use storage::get_token;

#[derive(Debug, Default, Clone)]
pub struct HttpCtx {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppResponse<S> {
    pub code: String,
    pub msg: String,
    pub data: Option<S>,
}

impl HttpCtx {
    pub async fn post<T, S>(&self, url: &str, params: &T) -> AppResult<Option<S>>
    where
        T: Serialize,
        S: DeserializeOwned,
    {
        let data: AppResponse<S> = Request::post(&format!("{}{}", CHRONO_IM_URL, url))
            .headers(self.gen_headers().await)
            .json(params)?
            .send()
            .await?
            .json()
            .await?;
        if !data.code.is_empty() {
            log!("{}", data.msg);
            return Err(CustomError(data.code));
        }
        Ok(data.data)
    }

    pub async fn gen_headers(&self) -> Headers {
        let header_map = Headers::new();
        header_map.set("Authorization", get_token().as_str());
        header_map.set("Content-Type", "application/json;charset=UTF-8");
        header_map
    }
}

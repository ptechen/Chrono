use axum::response::{IntoResponse, Response};
use axum::Json;
use error::error::ErrTrait;
use error::result::AppResult as Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl Default for AppResponse<()> {
    fn default() -> Self {
        AppResponse {
            code: String::new(),
            msg: String::new(),
            data: None,
        }
    }
}

impl<T> AppResponse<T>
where
    T: Serialize + Debug,
{
    pub fn ok(data: T) -> AppResponse<T> {
        AppResponse::<T> {
            code: String::new(),
            msg: String::new(),
            data: Some(data),
        }
    }

    pub fn err<E: ErrTrait>(err: &E) -> AppResponse<T> {
        tracing::error!("status:{} message:{}", err.to_code(), err.to_string());
        AppResponse::<T> {
            code: err.to_code(),
            msg: err.to_string(),
            data: None,
        }
    }

    pub async fn res(f: impl Future<Output = Result<T>>) -> AppResponse<T> {
        match f.await {
            Ok(d) => {
                tracing::debug!("data: {:?}", d);
                AppResponse::ok(d)
            }
            Err(e) => AppResponse::err(&e),
        }
    }
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

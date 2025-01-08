use super::response::AppResponse;
use crate::user::User;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use error::error::{AppError, ErrTrait};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HttpCtx {
    pub user: User,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Auth<T: Send + Sync>(pub T);

impl<S> FromRequestParts<S> for Auth<HttpCtx>
where
    S: Send + Sync,
{
    type Rejection = AppResponse<()>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut http_ctx = HttpCtx::default();
        let headers = &mut parts.headers;
        if let Some(authorization) = headers.get(header::AUTHORIZATION) {
            if let Ok(authorization) = authorization.to_str() {
                if let Ok(jwt) = User::decode(authorization).await {
                    http_ctx.user = jwt;
                } else {
                    return Err(AppResponse::err(&AppError::TokenIsInvalid.to_err()));
                }
            } else {
                return Err(AppResponse::err(&AppError::TokenIsNotExist.to_err()));
            }
        } else {
            return Err(AppResponse::err(&AppError::TokenIsNotExist.to_err()));
        }
        return Ok(Auth(http_ctx));
    }
}

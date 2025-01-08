use libp2p::identity::ParseError;
use std::fmt::Display;
use thiserror::__private::AsDisplay;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0}")]
    CustomError(String),
    #[error("ApiNotFound")]
    ApiNotFound,
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
    #[error(transparent)]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    AxumMissingJsonContentType(#[from] axum::extract::rejection::MissingJsonContentType),
    #[error(transparent)]
    AxumMissingPathParams(#[from] axum::extract::rejection::MissingPathParams),

    #[error(transparent)]
    AxumJsonSyntaxError(#[from] axum::extract::rejection::JsonSyntaxError),
    #[error("TokenIsExpired")]
    TokenIsExpired,
    #[error("TokenIsNotExist")]
    TokenIsNotExist,
    #[error("TokenIsInvalid")]
    TokenIsInvalid,
    #[error("UserIsNotExist")]
    UserIsNotExist,
    #[error("ChatSendError")]
    ChatSendError,
    #[error(transparent)]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    DecodingError(#[from] libp2p::identity::DecodingError),
    #[error(transparent)]
    SubscriptionError(#[from] libp2p::gossipsub::SubscriptionError),
    #[error(transparent)]
    MultiaddrError(#[from] libp2p::multiaddr::Error),
    #[error(transparent)]
    TransportError(#[from] libp2p::TransportError<std::io::Error>),
    #[error(transparent)]
    Libp2pNoiseError(#[from] libp2p::noise::Error),
    #[error(transparent)]
    SigningError(#[from] libp2p::identity::SigningError),
    #[error("peer_id:{0}NotFriend")]
    NotFriend(String),
    #[error(transparent)]
    IdentityParseError(#[from] libp2p::identity::ParseError),
}

pub trait ErrTrait: Display {
    fn to_code(&self) -> String;

    fn match_error(&self) -> (String, String);

    fn to_err(self) -> AppError
    where
        Self: Sized,
    {
        AppError::CustomError(self.to_string())
    }
}

impl ErrTrait for AppError {
    fn to_code(&self) -> String {
        let v = format!("{:?}", self.as_display());
        let v: Vec<&str> = v.split("(").collect();
        format!("{}Code", v.get(0).unwrap_or(&""))
    }

    fn match_error(&self) -> (String, String) {
        (self.to_code(), self.to_string())
    }

    fn to_err(self) -> AppError {
        self
    }
}

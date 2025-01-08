#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    GlooNet(#[from] gloo_net::Error),
    #[error("{0}")]
    CustomError(String),
}

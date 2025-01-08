use crate::error::AppError;

pub type AppResult<T> = Result<T, AppError>;

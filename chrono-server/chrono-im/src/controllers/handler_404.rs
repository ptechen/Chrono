use axum::response::IntoResponse;
use error::error::AppError;
use middleware::response::AppResponse;

pub async fn handler_404() -> impl IntoResponse {
    AppResponse::<()>::err(&AppError::ApiNotFound)
}

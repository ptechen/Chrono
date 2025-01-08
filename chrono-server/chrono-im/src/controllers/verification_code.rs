use crate::logics::verification_code::{SendVerificationCodeReq, VerifyVerificationCodeReq};
use axum::response::IntoResponse;
use axum::Json;
use middleware::response::AppResponse;

/// 发送验证码
pub async fn send_verification_code(
    Json(params): Json<SendVerificationCodeReq>,
) -> impl IntoResponse {
    AppResponse::res(params.send_verification_code()).await
}

/// 验证验证码
pub async fn verify_verification_code(
    Json(params): Json<VerifyVerificationCodeReq>,
) -> impl IntoResponse {
    AppResponse::res(params.verify_verification_code()).await
}

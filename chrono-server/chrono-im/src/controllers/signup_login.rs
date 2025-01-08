use crate::logics::signup_login::login::LoginReq;
use crate::logics::signup_login::reset_password::ResetPasswordReq;
use crate::logics::signup_login::signup::SignupReq;
use axum::response::IntoResponse;
use axum::Json;
use middleware::response::AppResponse;

/// 注册
pub async fn signup(Json(params): Json<SignupReq>) -> impl IntoResponse {
    AppResponse::res(params.signup()).await
}

/// 登录
pub async fn login(Json(params): Json<LoginReq>) -> impl IntoResponse {
    AppResponse::res(params.login()).await
}

/// 重置密码
pub async fn reset_password(Json(params): Json<ResetPasswordReq>) -> impl IntoResponse {
    AppResponse::res(params.reset_password()).await
}

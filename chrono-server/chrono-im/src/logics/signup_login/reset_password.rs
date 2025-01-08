use crate::logics::verification_code::VerifyVerificationCodeReq;
use error::result::AppResult;
use models::owner_info::OwnerInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResetPasswordReq {
    pub email: String,
    pub code: String,
    pub password: String,
}

impl ResetPasswordReq {
    pub async fn reset_password(&self) -> AppResult<()> {
        VerifyVerificationCodeReq {
            code: self.code.to_string(),
        }
        .verify_verification_code()
        .await?;
        OwnerInfo::update_password_by_email(&self.email, &self.password).await?;
        Ok(())
    }
}

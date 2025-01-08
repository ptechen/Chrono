use chrono::Utc;
use error::error::AppError::CustomError;
use error::result::AppResult;
use models::verification_code::VerificationCode;
use rand::Rng;
use send_email::send_email::SendEmail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendVerificationCodeReq {
    pub to: String,
}

impl SendVerificationCodeReq {
    pub async fn send_verification_code(&self) -> AppResult<()> {
        let mut codes = vec![];
        for _ in 0..6 {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..=9);
            codes.push(random_number.to_string());
        }
        let code = codes.join("");
        SendEmail {
            subject: "VerificationCode".to_string(),
            to: self.to.to_owned(),
            content: code.to_owned(),
        }
        .send_email()
        .await?;
        VerificationCode {
            code,
            created_at: Utc::now().timestamp(),
        }
        .insert()
        .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyVerificationCodeReq {
    pub code: String,
}

impl VerifyVerificationCodeReq {
    pub async fn verify_verification_code(&self) -> AppResult<()> {
        if let Ok(Some(code)) = VerificationCode::select_optional_by_code(&self.code).await {
            if Utc::now().timestamp() - code.created_at > 10 * 60 {
                return Err(CustomError("Verification code has expired".to_string()));
            }
            return Ok(());
        }
        return Err(CustomError("Verification code does not exist".to_string()));
    }
}

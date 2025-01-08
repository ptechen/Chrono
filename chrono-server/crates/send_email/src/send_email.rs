use error::result::AppResult;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct SendEmail {
    pub subject: String,
    pub to: String,
    pub content: String,
}

impl SendEmail {
    pub async fn send_email(&self) -> AppResult<()> {
        let email = Message::builder()
            .from("15736755067@163.com".parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(&self.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(self.content.to_owned())?;

        let creds = Credentials::new(
            "15736755067@163.com".to_owned(),
            "BBDAUMEAOTXQUZWR".to_owned(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.163.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        mailer.send(&email)?;
        Ok(())
    }
}

use send_email::send_email::SendEmail;

#[tokio::test]
async fn test_email() {
    if let Err(e) = (SendEmail {
        to: "540123685@qq.com".to_string(),
        content: "test".to_string(),
    }
    .send_email()
    .await)
    {
        println!("{e}");
    }
}

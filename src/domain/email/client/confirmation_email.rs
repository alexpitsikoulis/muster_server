use super::Client;
use crate::domain::user;
use secrecy::{ExposeSecret, Secret};

impl Client {
    pub async fn send_confirmation_email(
        &self,
        recipient: user::Email,
        confirmation_token: &Secret<String>,
    ) -> Result<(), reqwest::Error> {
        let subject = "Confirm Your Email";
        let body = format!(
            r#"<a href="/confirm/{}""#,
            confirmation_token.expose_secret()
        );
        self.send_email(
            recipient,
            subject.to_string(),
            body.to_string(),
            String::new(),
        )
        .await
    }
}

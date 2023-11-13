use secrecy::{Secret, ExposeSecret};
use crate::domain::user;
use super::Client;

impl Client {
    pub async fn send_confirmation_email(
        &self,
        recipient: user::Email,
        confirmation_token: Secret<String>,
    ) -> Result<(), reqwest::Error> {
        let subject = "Confirm Your Email";
        let body = format!("<a href = {}/confirm/{}", self.base_url, confirmation_token.expose_secret() );
        self.send_email(recipient, subject.to_string(), body.to_string(), String::new()).await
    }
}
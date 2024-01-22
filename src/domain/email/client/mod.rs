mod confirmation_email;
mod tests;

use super::ServerEmail;
use crate::domain::user;

#[derive(Debug)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    sender: user::Email,
}

impl Client {
    pub fn new(base_url: String, sender: user::Email) -> Self {
        Client {
            http_client: reqwest::Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: user::Email,
        subject: String,
        html_content: String,
        text_content: String,
    ) -> Result<(), reqwest::Error> {
        self.http_client
            .post(format!("{}/send", self.base_url))
            .json(&ServerEmail::new(
                self.sender.clone(),
                recipient,
                subject,
                html_content,
                text_content,
            ))
            .send()
            .await?;
        Ok(())
    }
}

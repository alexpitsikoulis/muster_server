mod tests;
mod confirmation_email;

use crate::domain::user;
use super::Email;

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
    ) -> Result<(), String> {
        match self.http_client
            .post(&format!("{}/send", self.base_url))
            .body(Email::new(self.sender.clone(), recipient, subject, html_content, text_content).to_string())
            .send()
            .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
    }
}
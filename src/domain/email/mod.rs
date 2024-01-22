mod client;
pub use client::*;

use crate::domain::user;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ServerEmail {
    sender: String,
    recipient: String,
    subject: String,
    body: String,
}

impl ServerEmail {
    pub fn new(
        sender: user::Email,
        recipient: user::Email,
        subject: String,
        html_content: String,
        text_content: String,
    ) -> Self {
        let sender = sender.as_ref().to_string();
        let recipient = recipient.as_ref().to_string();
        ServerEmail {
            sender,
            recipient,
            subject,
            body: format!("{}{}", html_content, text_content),
        }
    }
}

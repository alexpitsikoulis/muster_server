mod client;
pub use client::*;

use crate::domain::user;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Email {
    sender: String,
    recipient: String,
    subject: String,
    body: String,
}

impl Email {
    pub fn new(
        sender: user::Email,
        recipient: user::Email,
        subject: String,
        html_content: String,
        text_content: String,
    ) -> Self {
        let sender = sender.as_ref().to_string();
        let recipient = recipient.as_ref().to_string();
        Email {
            sender: sender,
            recipient: recipient,
            subject: subject,
            body: format!("{}{}", html_content, text_content),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            r#"
                {{
                    "sender": "{}",
                    "recipient": "{}",
                    "subject": "{}",
                    "body": "{}"
                }}
            "#,
            self.sender, self.recipient, self.subject, self.body,
        )
    }
}

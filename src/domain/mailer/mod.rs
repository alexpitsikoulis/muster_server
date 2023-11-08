use reqwest::Client;
use super::user::Email;

pub struct Mailer {
    http_client: Client,
    base_url: String,
    sender: Email,
}

impl Mailer {
    pub fn new(base_url: String, sender: Email) -> Self {
        Mailer {
            http_client: reqwest::Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &str,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
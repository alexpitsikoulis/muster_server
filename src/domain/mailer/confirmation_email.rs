use actix_web::web::Data;
use crate::config::Config;
use super::Mailer;

impl Mailer {
    pub async fn send_confirmation_email(
        &self,
        recipient: &str,
        confirmation_id: String,
        config: Data<Config>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sender = "no-reply@muttr.com";
        let subject = "Confirm Your Email";
        let body = format!("<a href = {}:{}/confirm/{}", config.app.host, config.app.port, confirmation_id);
        self.send(sender, recipient, subject, body.to_string()).await
    }
}
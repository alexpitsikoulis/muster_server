use actix_web::web::Data;
use crate::{
    config::Config,
    domain::user,
};
use super::Client;

impl Client {
    pub async fn send_confirmation_email(
        &self,
        recipient: user::Email,
        confirmation_id: String,
        config: Data<Config>
    ) -> Result<(), String> {
        let subject = "Confirm Your Email";
        let body = format!("<a href = {}:{}/confirm/{}", config.app.host, config.app.port, confirmation_id);
        self.send_email(recipient, subject.to_string(), body.to_string(), String::new()).await
    }
}
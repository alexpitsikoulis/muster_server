mod confirmation_email;

use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

#[derive(Debug)]
pub enum Error {
    AddressError(lettre::address::AddressError),
    EmailError(lettre::transport::smtp::Error),
    Error(lettre::error::Error),
}

#[derive(Clone)]
pub struct Mailer(AsyncSmtpTransport<Tokio1Executor>);

impl Mailer {
    pub fn new(address: String, credentials: Credentials) -> Result<Self, Error> {
        match AsyncSmtpTransport::<Tokio1Executor>::relay(&address) {
            Ok(builder) => Ok(Mailer(builder.credentials(credentials).build())),
            Err(e) => Err(Error::EmailError(e)),
        }
    }

    pub async fn send(
        &self,
        sender: &str,
        recipient: &str,
        subject: &str,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(sender.parse()?)
            .to(recipient.parse()?)
            .subject(subject)
            .body(body)?;
    
        self.0.send(email).await?;
        Ok(())
    }
}
mod tests;

use secrecy::{Secret, ExposeSecret};
use uuid::Uuid;

pub struct ConfirmationToken {
    confirmation_token: Secret<String>,
    user_id: Uuid,
}

impl ConfirmationToken {
    pub fn new(confirmation_token: Secret<String>, user_id: Uuid) -> Self {
        ConfirmationToken { confirmation_token, user_id }
    }

    pub fn expose(&self) -> String {
        self.confirmation_token.expose_secret().clone()
    }

    pub fn confirmation_token(&self) -> &Secret<String> {
        &self.confirmation_token
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn set_confirmation_token(&mut self, confirmation_token: Secret<String>) {
        self.confirmation_token = confirmation_token;
    }

    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }
}
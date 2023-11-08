mod tests;

use std::str::FromStr;
use actix_web::web::Query;
use secrecy::{Secret, ExposeSecret};
use uuid::Uuid;
use crate::handlers::user::GetConfirmationTokenData;

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

    pub fn confirmation_token(&self) -> Secret<String> {
        self.confirmation_token.clone()
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

impl TryFrom<Query<GetConfirmationTokenData>> for ConfirmationToken {
    type Error = String;

    fn try_from(value: Query<GetConfirmationTokenData>) -> Result<Self, Self::Error> {
        match Uuid::from_str(&value.user_id) {
            Ok(user_id) => Ok(ConfirmationToken::new(
                value.confirmation_token.clone(),
                user_id,
            )),
            Err(e) => Err(format!("Failed to parse user_id: {:?}", e))
        }
    }
}

use std::str::FromStr;
use actix_web::web::Query;
use secrecy::Secret;
use uuid::Uuid;

use crate::storage::ConfirmationToken;

#[derive(serde::Deserialize, Clone)]
pub struct GetConfirmationTokenData {
    pub confirmation_token: Secret<String>,
    pub user_id: String,
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

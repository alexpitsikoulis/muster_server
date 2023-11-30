use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Owner,
    Admin,
    Moderator,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub allowed_servers: HashMap<String, Role>,
}

#[derive(Debug)]
pub enum Error {
    EncodeErr(String),
    DecodeErr(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn generate_token(user_id: Uuid) -> Result<String> {
    let current_time = chrono::Utc::now();
    let claims = Claims {
        exp: match current_time.checked_add_days(chrono::Days::new(14)) {
            Some(new_date) => new_date.timestamp() as usize,
            None => {
                return Err(Error::EncodeErr(String::from(
                    "Failed to generate exp date for JWT",
                )))
            }
        },
        iat: current_time.timestamp() as usize,
        sub: user_id.to_string(),
        allowed_servers: HashMap::new(),
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ) {
        Ok(token) => Ok(token),
        Err(e) => Err(Error::EncodeErr(e.to_string())),
    }
}

pub fn get_claims_from_token(token: String) -> Result<Claims> {
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(Error::DecodeErr(e.to_string())),
    }
}

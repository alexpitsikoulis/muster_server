mod email;
mod handle;
mod password;
mod login;

use actix_web::{web::Form, HttpResponse};
pub use handle::*;
pub use password::*;
pub use email::*;
pub use login::*;

use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::handlers::SignupFormData;

#[derive(Debug)]
pub enum UserValidationError {
    EmailValidationErr(EmailValidationErr),
    HandleValidationErr(HandleValidationErr),
    PasswordValidationErr(PasswordValidationErr),
}

impl UserValidationError {
    pub fn handle_http(&self) -> HttpResponse {
        let body = match self {
            Self::EmailValidationErr(e) => format!("Email is not valid: {:?}", e),
            Self::HandleValidationErr(e) => match e {
                HandleValidationErr::HandleEmpty => String::from("User handle is empty"),
                HandleValidationErr::HandleContainsWhiteSpace => String::from("User handle may not contain whitespace characters"),
                HandleValidationErr::HandleTooLong => String::from("User handle is too long, must be no more than 20 characters"),
                HandleValidationErr::HandleContainsForbiddenChars(c) => format!("User handle contains forbidden character '{}'", c),
            },
            Self::PasswordValidationErr(e) => match e {
                PasswordValidationErr::PwdTooShort => String::from("Password is too short, must be no shorter than 8 characters"),
                PasswordValidationErr::PwdTooLong => String::from("Password is too long, must be no more than 64 characters"),
                PasswordValidationErr::PwdMissingLowercase => String::from("Password must contain at least one lowercase letter"),
                PasswordValidationErr::PwdMissingUppercase => String::from("Password must contain at least one uppsercase letter"),
                PasswordValidationErr::PwdMissingNumber => String::from("Password must contain at least one number"),
                PasswordValidationErr::PwdMissingChar => String::from("Password must contain at least one special character (\" # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \\ ] ^ _ ` { | } ~ )"),
                PasswordValidationErr::ArgonErr(e) => {
                    tracing::error!("Argon2 failed to hash password: {:?}", e);
                    return HttpResponse::InternalServerError().finish()
                },
            }
        };
        HttpResponse::BadRequest().body(body)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub handle: String,
    pub name: Option<String>,
    pub password: String,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
    pub failed_attempts: i16,
    pub email_confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        handle: String,
        name: Option<String>,
        password: String,
        profile_photo: Option<String>,
        bio: Option<String>,
        failed_attempts: i16,
        email_confirmed: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        User {
            id,
            email,
            handle,
            name,
            password,
            profile_photo,
            bio,
            failed_attempts,
            email_confirmed,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

impl TryFrom<Form<SignupFormData>> for User {
    type Error = UserValidationError;
    fn try_from(form: Form<SignupFormData>) -> Result<Self, Self::Error> {
        let email = match Email::parse(form.email.clone()) {
            Ok(e) => e,
            Err(e) => return Err(UserValidationError::EmailValidationErr(e)),
        };
        let handle = match Handle::parse(form.handle.clone()) {
            Ok(h) => h,
            Err(e) => return Err(UserValidationError::HandleValidationErr(e)),
        };
        let password = match Password::parse(form.password.clone()) {
            Ok(p) => p,
            Err(e) => return Err(UserValidationError::PasswordValidationErr(e)),
        };

        let now = Utc::now();

        Ok(User::new(
            Uuid::new_v4(),
            email.as_ref().to_string(),
            handle.as_ref().to_string(),
            None,
            password.as_ref().to_string(),
            None,
            None,
            0,
            false,
            now,
            now,
            None,
        ))
    }
}
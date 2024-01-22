mod credentials;
mod tests;

use actix_web::HttpResponse;
pub use credentials::{
    deserialize_handle_option, deserialize_password_option, deserilaize_email_option, Email,
    EmailValidationErr, Handle, HandleValidationErr, Login, Password, PasswordValidationErr,
    ALLOWED_HANDLE_CHARS, ALLOWED_PASSWORD_CHARS,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::handlers::user::UserSignupFormData;

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

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct User {
    #[serde(default)]
    id: Uuid,
    email: Email,
    handle: Handle,
    password: Password,
    name: Option<String>,
    profile_photo: Option<String>,
    bio: Option<String>,
    failed_attempts: i16,
    email_confirmed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.email == other.email
            && self.handle == other.handle
            && self.name == other.name
            && self.profile_photo == other.profile_photo
            && self.bio == other.bio
            && self.failed_attempts == other.failed_attempts
            && self.email_confirmed == other.email_confirmed
            && self.deleted_at == other.deleted_at
    }
}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        email: Email,
        handle: Handle,
        password: Password,
        name: Option<String>,
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
            password,
            name,
            profile_photo,
            bio,
            failed_attempts,
            email_confirmed,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }

    pub fn profile_photo(&self) -> Option<String> {
        self.profile_photo.clone()
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    pub fn failed_attempts(&self) -> i16 {
        self.failed_attempts
    }

    pub fn email_confirmed(&self) -> bool {
        self.email_confirmed
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn set_email(&mut self, email: Email) {
        self.email = email;
    }

    pub fn set_email_string(&mut self, email: String) -> Result<(), EmailValidationErr> {
        match Email::try_from(email.as_str()) {
            Ok(email) => {
                self.email = email;
                Ok(())
            }
            Err(e) => {
                tracing::error!("User email {} is invalid: {:?}", email, e);
                Err(e)
            }
        }
    }

    pub fn set_handle(&mut self, handle: Handle) {
        self.handle = handle;
    }

    pub fn set_handle_string(&mut self, handle: String) -> Result<(), HandleValidationErr> {
        match Handle::try_from(handle.as_str()) {
            Ok(handle) => {
                self.handle = handle;
                Ok(())
            }
            Err(e) => {
                tracing::error!("User handle {} is invalid: {:?}", handle, e);
                Err(e)
            }
        }
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name
    }

    pub fn set_password(&mut self, password: Password) {
        self.password = password;
    }

    pub fn set_profile_photo(&mut self, profile_photo: Option<String>) {
        self.profile_photo = profile_photo
    }

    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio
    }

    pub fn increment_failed_attempts(&mut self) {
        self.failed_attempts += 1;
    }

    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
    }

    pub fn set_email_confirmed(&mut self, email_confirmed: bool) {
        self.email_confirmed = email_confirmed;
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at
    }

    pub fn set_deleted_at(&mut self, deleted_at: Option<DateTime<Utc>>) {
        self.deleted_at = deleted_at;
    }
}

impl From<UserSignupFormData> for User {
    fn from(form: UserSignupFormData) -> Self {
        let now = Utc::now();
        User {
            id: form.id,
            email: form.email,
            handle: form.handle,
            password: form.password,
            name: None,
            profile_photo: None,
            bio: None,
            failed_attempts: 0,
            email_confirmed: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

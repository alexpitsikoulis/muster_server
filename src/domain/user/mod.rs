mod email;
mod handle;
mod password;
mod login;
mod tests;

use actix_web::{web::Form, HttpResponse};
pub use handle::*;
pub use password::*;
pub use email::*;
pub use login::*;

use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::handlers::user::SignupFormData;

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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn handle(&self) -> String {
        self.handle.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn password(&self) -> String {
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
        self.email = email.as_ref().to_string();
    }

    pub fn set_handle(&mut self, handle: Handle) {
        self.handle = handle.as_ref().to_string();
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name
    }

    pub fn set_password(&mut self, password: Password) {
        self.password = password.as_ref().to_string();
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
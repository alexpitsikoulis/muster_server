mod email;
mod handle;
mod password;

pub use handle::HandleValidationErr;
pub use password::{PasswordValidationErr, UserPassword};

use email::*;
use handle::*;

use actix_web::{web::Form, HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use crate::{
    handlers::SignupFormData,
    storage::User
};



#[derive(Debug)]
pub enum UserValidationErr {
    EmailValidationErr(EmailValidationErr),
    HandleValidationErr(HandleValidationErr),
    PasswordValidationErr(PasswordValidationErr),
}

impl UserValidationErr {
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

pub struct NewUser {
    pub email: UserEmail,
    pub handle: UserHandle,
    pub password: UserPassword,
}

impl NewUser {
    pub fn parse(form: Form<SignupFormData>) -> Self {
        match Self::try_parse(form) {
            Ok(u) => u,
            Err(e) => panic!("Failed to generate NewUser: {:?}", e),
        }
    }

    pub fn try_parse(form: Form<SignupFormData>) -> Result<Self, UserValidationErr>  {
        let email = match UserEmail::try_parse(form.email.clone()) {
            Ok(e) => e,
            Err(e) => return Err(UserValidationErr::EmailValidationErr(e)),
        };
        let handle = match UserHandle::try_parse(form.handle.clone()) {
            Ok(h) => h,
            Err(e) => return Err(UserValidationErr::HandleValidationErr(e)),
        };
        let password = match UserPassword::try_parse(form.password.clone()) {
            Ok(p) => p,
            Err(e) => return Err(UserValidationErr::PasswordValidationErr(e)),
        };

        Ok(NewUser{email, handle, password})
    }
}

impl Into<User> for NewUser {
    fn into(self) -> User {
        let now = Utc::now();
        User::new(
            Uuid::new_v4(),
            self.email.as_ref().to_string(),
            self.handle.as_ref().to_string(),
            None,
            self.password.as_ref().to_string(),
            None,
            None,
            0,
            now,
            now,
            None,
        )
    }
}
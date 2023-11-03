mod email;
mod handle;
mod password;

pub use handle::{HandleValidationErr, ALLOWED_HANDLE_CHARS};
pub use password::{PasswordValidationErr, UserPassword, ALLOWED_PASSWORD_CHARS};

use email::*;
use handle::*;

use actix_web::{
    HttpResponse,
    web:: Form,
};
use crate::handlers::SignupFormData;


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

impl TryFrom<Form<SignupFormData>> for NewUser {
    type Error = UserValidationErr;

    fn try_from(form: Form<SignupFormData>) -> Result<Self, Self::Error> {
        let email = match UserEmail::parse(form.email.clone()) {
            Ok(e) => e,
            Err(e) => return Err(UserValidationErr::EmailValidationErr(e)),
        };
        let handle = match UserHandle::parse(form.handle.clone()) {
            Ok(h) => h,
            Err(e) => return Err(UserValidationErr::HandleValidationErr(e)),
        };
        let password = match UserPassword::parse(form.password.clone()) {
            Ok(p) => p,
            Err(e) => return Err(UserValidationErr::PasswordValidationErr(e)),
        };

        Ok(NewUser{email, handle, password})
    }
}
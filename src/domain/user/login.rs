use actix_web::web::Form;
use secrecy::Secret;

use crate::handlers::user::LoginForm;

use super::{Email, Handle};

#[derive(serde::Deserialize, Clone)]
pub enum Login {
    Email(String),
    Handle(String),
}

impl std::fmt::Display for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (login, value) = match self {
            Self::Email(e) => ("email", e),
            Self::Handle(h) => ("handle", h),
        };
        write!(f, "{}: {}", login, value)
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct LoginData {
    pub login: Login,
    pub password: Secret<String>,
}

impl TryFrom<Form<LoginForm>> for LoginData {
    type Error = String;

    fn try_from(form: Form<LoginForm>) -> Result<Self, Self::Error> {
        match Email::parse(form.login.clone()) {
            Ok(e) => Ok(LoginData {
                login: Login::Email(e.as_ref().to_string()),
                password: form.password.clone(),
            }),
            Err(_) => match Handle::parse(form.login.clone()) {
                Ok(h) => Ok(LoginData {
                    login: Login::Handle(h.as_ref().to_string()),
                    password: form.password.clone(),
                }),
                Err(_) => Err("Email/Handle invalid".to_string()),
            },
        }
    }
}

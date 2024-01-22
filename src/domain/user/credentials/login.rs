use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

use super::{Email, Handle};

#[derive(Serialize, Clone)]
pub enum Login {
    Email(Email),
    Handle(Handle),
}

impl std::fmt::Display for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let login = match self {
            Self::Email(e) => e.as_ref(),
            Self::Handle(h) => h.as_ref(),
        };
        write!(f, "{}", login)
    }
}

impl<'de> Deserialize<'de> for Login {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(LoginVisitor)
    }
}

struct LoginVisitor;

impl<'de> Visitor<'de> for LoginVisitor {
    type Value = Login;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a valid email or user handle")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Email::try_from(v.clone()) {
            Ok(email) => Ok(Login::Email(email)),
            Err(_) => match Handle::try_from(v.clone()) {
                Ok(handle) => Ok(Login::Handle(handle)),
                Err(_) => Err(E::invalid_type(Unexpected::Str(&v), &self)),
            },
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }
}

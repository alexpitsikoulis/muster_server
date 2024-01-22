use regex::Regex;
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug)]
pub enum EmailValidationErr {
    EmailInvalidErr(String),
}

#[derive(Debug, Clone, PartialEq)]

pub struct Email(String);

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    fn email_regex() -> Regex {
        Regex::new(r"^[a-zA-Z0-9]{1}[\w\.\-]*[a-zA-Z0-9]+@[a-zA-Z0-9]{1}\.?(([\w\-]+)(\.?[a-zA-Z0-9]))+\.[a-zA-Z0-9]{2,4}$").unwrap()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = EmailValidationErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailValidationErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if Self::email_regex().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(EmailValidationErr::EmailInvalidErr(format!(
                "Email {} is invalid",
                value
            )))
        }
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(EmailVisitor)
    }
}

struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = Email;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a valid email address")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Email::try_from(v.as_str()).map_err(|_| E::invalid_value(Unexpected::Str(&v), &self))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }
}

pub struct EmailOptionVisitor;

impl<'de> Visitor<'de> for EmailOptionVisitor {
    type Value = Option<Email>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a valid email address")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Some(Email::try_from(v.as_str()).map_err(|_| {
            E::invalid_value(Unexpected::Str(&v), &self)
        })?))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(EmailOptionVisitor)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

pub fn deserilaize_email_option<'de, D>(deserializer: D) -> Result<Option<Email>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(EmailOptionVisitor)
}

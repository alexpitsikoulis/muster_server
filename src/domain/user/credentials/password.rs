use argon2::{self, Config};
use rand::RngCore;
use secrecy::{ExposeSecret, Secret};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

pub const ALLOWED_PASSWORD_CHARS: &[char] = &[
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
    '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

#[derive(Debug)]
pub enum PasswordValidationErr {
    PwdTooShort,
    PwdTooLong,
    PwdMissingLowercase,
    PwdMissingUppercase,
    PwdMissingNumber,
    PwdMissingChar,
    ArgonErr(argon2::Error),
}

#[derive(Clone, Debug)]
pub struct Password(Secret<String>);

impl Password {
    pub fn from_raw(password: Secret<String>) -> Self {
        Password(password)
    }

    fn validate(password: Secret<String>) -> Result<(), PasswordValidationErr> {
        if password.expose_secret().len() < 8 {
            return Err(PasswordValidationErr::PwdTooShort);
        }

        if password.expose_secret().len() > 64 {
            return Err(PasswordValidationErr::PwdTooLong);
        }

        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_number = false;
        let mut has_char = false;

        for c in password.expose_secret().chars() {
            if has_lower && has_upper && has_number && has_char {
                break;
            }
            if !has_lower && c.is_ascii_lowercase() {
                has_lower = true;
                continue;
            }
            if !has_upper && c.is_ascii_uppercase() {
                has_upper = true;
                continue;
            }
            if !has_number && c.is_ascii_digit() {
                has_number = true;
                continue;
            }
            if !has_char && ALLOWED_PASSWORD_CHARS.contains(&c) {
                has_char = true;
                continue;
            }
        }

        if !has_lower {
            return Err(PasswordValidationErr::PwdMissingLowercase);
        }
        if !has_upper {
            return Err(PasswordValidationErr::PwdMissingUppercase);
        }
        if !has_number {
            return Err(PasswordValidationErr::PwdMissingNumber);
        }
        if !has_char {
            return Err(PasswordValidationErr::PwdMissingChar);
        }

        Result::Ok(())
    }

    pub fn compare(&self, password: &Secret<String>) -> bool {
        argon2::verify_encoded(self.as_ref(), password.expose_secret().as_bytes()).unwrap()
    }

    pub fn inner(&self) -> &Secret<String> {
        &self.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl TryFrom<Secret<String>> for Password {
    type Error = PasswordValidationErr;

    fn try_from(value: Secret<String>) -> Result<Self, Self::Error> {
        match Self::validate(value.clone()) {
            Ok(()) => {
                let salt = {
                    let mut unencoded = [0u8; 16];
                    let mut rng = rand::thread_rng();
                    rng.fill_bytes(&mut unencoded);
                    unencoded
                };

                let config = Config::original();

                match argon2::hash_encoded(value.expose_secret().as_bytes(), &salt, &config) {
                    Ok(hash) => Ok(Self(Secret::new(hash))),
                    Err(e) => Err(PasswordValidationErr::ArgonErr(e)),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PasswordVisitor)
    }
}

struct PasswordVisitor;

impl<'de> Visitor<'de> for PasswordVisitor {
    type Value = Password;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "an 8-64 character string containing at least one uppercase letter, one lowercase letter, one number, and one special character")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() > 64
            && (v.starts_with("$argon2i$")
                || v.starts_with("$argon2d$")
                || v.starts_with("$argon2id$"))
        {
            Ok(Password::from_raw(Secret::new(v)))
        } else {
            Password::try_from(Secret::new(v.clone()))
                .map_err(|_| E::invalid_value(Unexpected::Str(&v), &self))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }
}

struct PasswordOptionVisitor;

impl<'de> Visitor<'de> for PasswordOptionVisitor {
    type Value = Option<Password>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "an 8-64 character string containing at least one uppercase letter, one lowercase letter, one number, and one special character")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with("$argon2i$") || v.starts_with("$argon2d$") || v.starts_with("$argon2id$") {
            Ok(Some(Password::from_raw(Secret::new(v))))
        } else {
            Ok(Some(Password::try_from(Secret::new(v.clone())).map_err(
                |_| E::invalid_value(Unexpected::Str(&v), &self),
            )?))
        }
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
        deserializer.deserialize_string(PasswordOptionVisitor)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

pub fn deserialize_password_option<'de, D>(deserializer: D) -> Result<Option<Password>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(PasswordOptionVisitor)
}

#[cfg(test)]
mod tests {
    use crate::{domain::user::Password, utils::test::PASSWORD_GENERATOR};
    use claim::assert_err;
    use secrecy::Secret;

    #[derive(Clone, Debug)]
    struct ValidPasswordFixture(pub Secret<String>);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let password = PASSWORD_GENERATOR.generate(g);
            ValidPasswordFixture(Secret::new(password))
        }
    }

    #[test]
    fn fails_when_less_than_8_grapheme() {
        let password = Secret::new("P@ssw0r".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_more_than_64_grapheme() {
        let filler = "A".repeat(60);
        let password = Secret::new(format!("P@ss1{}", filler).to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_uppercase() {
        let password = Secret::new("n0neofyourbus!ness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_lowercase() {
        let password = Secret::new("N0NEOFYOURBUS!NESS".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_number() {
        let password = Secret::new("Noneofyourbus!ness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[test]
    fn fails_when_no_special_char() {
        let password = Secret::new("N0neofyourbusiness".to_string());
        assert_err!(Password::try_from(password));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_password_parses_successfully(password: ValidPasswordFixture) -> bool {
        Password::try_from(password.0).is_ok()
    }
}

use regex::Regex;

#[derive(Debug)]
pub enum EmailValidationErr {
    EmailInvalidErr(String)
}

pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(email: String) -> Self {
        match Self::try_parse(email) {
            Ok(e) => e,
            Err(e) => panic!("Email validation failed: {:?}", e),
        }
    }
    
    pub fn try_parse(email: String) -> Result<Self, EmailValidationErr> {
        if Self::email_regex().is_match(email.as_str()) {
            Ok(Self(email))
        } else {
            Err(EmailValidationErr::EmailInvalidErr(format!("Email {} is invalid", email)))
        }
    }

    fn email_regex() -> Regex {
        Regex::new(r"^[a-zA-Z0-9]{1}[\w\.\-]*[a-zA-Z]+@[a-zA-Z0-9]{1}\.?(([\w\-]+)(\.?[a-zA-Z0-9]))+\.[a-zA-Z0-9]{2,4}$").unwrap()
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
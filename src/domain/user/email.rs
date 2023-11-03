use regex::Regex;

#[derive(Debug)]
pub enum EmailValidationErr {
    EmailInvalidErr(String)
}

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(email: String) -> Result<Self, EmailValidationErr> {
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

#[cfg(test)]
mod tests {
    use crate::domain::user::UserEmail;
    use fake::{
        Fake,
        faker::internet::en::SafeEmail,
    };
    use claim::assert_err;

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            ValidEmailFixture(email)
        }
    }
    #[test]
    fn invalid_email_rejected() {
        let emails = &[
            "alex",
            "alex.pitsikoulis@test",
            "alex@test.",
            "alex@test.qwertyuiop",
            "_alex@test.com",
            "alex_@test.com",
            "alex@test.com_",
            "alex@_test.com",
        ];
        for email in emails {
            assert_err!(UserEmail::parse(email.to_string()));
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_parsed_successfully(email: ValidEmailFixture) -> bool {
        UserEmail::parse(email.0).is_ok()
    }
}
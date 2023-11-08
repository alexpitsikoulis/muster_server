#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use fake::{
        Fake,
        faker::internet::en::SafeEmail,
    };
    use crate::{
        utils::test::{PASSWORD_GENERATOR, HANDLE_GENERATOR},
        domain::user::{email::Email, Password, Handle},
    };
    use secrecy::Secret;
    
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
            assert_err!(Email::parse(email.to_string()));
        }
    }
    
    #[quickcheck_macros::quickcheck]
    fn valid_email_parsed_successfully(email: ValidEmailFixture) -> bool {
        Email::parse(email.0).is_ok()
    }

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
        assert_err!(Password::parse(password));
    }

    #[test]
    fn fails_when_more_than_64_grapheme() {
        let filler = "A".repeat(60);
        let password = Secret::new(format!("P@ss1{}", filler).to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn fails_when_no_uppercase() {
        let password = Secret::new("n0neofyourbus!ness".to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn fails_when_no_lowercase() {
        let password = Secret::new("N0NEOFYOURBUS!NESS".to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn fails_when_no_number() {
        let password = Secret::new("Noneofyourbus!ness".to_string());
        assert_err!(Password::parse(password));
    }

    #[test]
    fn fails_when_no_special_char() {
        let password = Secret::new("N0neofyourbusiness".to_string());
        assert_err!(Password::parse(password));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_password_parses_successfully(password: ValidPasswordFixture) -> bool {
        Password::parse(password.0).is_ok()
    }

    #[derive(Clone,Debug)]
    struct ValidHandleFixture(pub String);

    impl quickcheck::Arbitrary for ValidHandleFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let handle = HANDLE_GENERATOR.generate(g);
            ValidHandleFixture(handle)
        }
    }

    #[test]
    fn a_20_grapheme_long_handle_is_valid() {
        let handle = "A".repeat(20);
        assert_ok!(Handle::parse(handle));
    }

    #[test]
    fn a_handle_longer_than_20_grapheme_is_rejected() {
        let handle = "A".repeat(21);
        assert_err!(Handle::parse(handle));
    }

    #[test]
    fn whitespace_only_handles_rejected() {
        let handles = &[
            "\t",
            " ",
            "   ",
            "\n",
            "
            ",
        ];

        for handle in handles {
            assert_err!(Handle::parse(handle.to_string()));
        }
    }

    #[test]
    fn handle_with_forbidden_characters_rejected() {
        let handles = &["/", "(", ")", "'", "\"", "<", ">", "\\", "{", "}"];
        for handle in handles {
            assert_err!(Handle::parse(handle.to_string()));
        }
    }
    
    #[test]
    fn empty_string_handle_rejected() {
        assert_err!(Handle::parse("".to_string()));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_handle_parsed_successfully(handle: ValidHandleFixture) -> bool {
        Handle::parse(handle.0).is_ok()
    }
}
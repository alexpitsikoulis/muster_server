#[derive(Debug)]
pub enum HandleValidationErr {
    HandleEmpty,
    HandleTooLong,
    HandleContainsWhiteSpace,
    HandleContainsForbiddenChars(char),
}

pub const ALLOWED_HANDLE_CHARS: &[char] =  &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '#', '$', '%', '&', '*', '+', ',', '-', '.', ':', ';', '=', '?', '@', '[', ']', '^', '_', '`', '|', '~'];

#[derive(Debug)]
pub struct UserHandle(String);

impl UserHandle {
    pub fn parse(handle: String) -> Result<Self, HandleValidationErr> {
        let mut forbidden_character: Option<char> = None;
        if handle.trim().is_empty() {
            return Err(HandleValidationErr::HandleEmpty)
        } else if handle.len() > 20 {
            return Err(HandleValidationErr::HandleTooLong)
        } else if handle
                .chars()
                .any(|c| {
                    let contains_forbidden = !ALLOWED_HANDLE_CHARS.contains(&c);

                    if contains_forbidden {
                        forbidden_character = Some(c);
                    }

                    contains_forbidden
                }) {
            return Err(HandleValidationErr::HandleContainsForbiddenChars(forbidden_character.unwrap()))
        }
        Ok(UserHandle(handle))
    }
}

impl AsRef<str> for UserHandle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::user::*,
        utils::test::HANDLE_GENERATOR,
    };
    use claim::{assert_err, assert_ok};

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
        assert_ok!(UserHandle::parse(handle));
    }

    #[test]
    fn a_handle_longer_than_20_grapheme_is_rejected() {
        let handle = "A".repeat(21);
        assert_err!(UserHandle::parse(handle));
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
            assert_err!(UserHandle::parse(handle.to_string()));
        }
    }

    #[test]
    fn handle_with_forbidden_characters_rejected() {
        let handles = &["/", "(", ")", "'", "\"", "<", ">", "\\", "{", "}"];
        for handle in handles {
            assert_err!(UserHandle::parse(handle.to_string()));
        }
    }
    
    #[test]
    fn empty_string_handle_rejected() {
        assert_err!(UserHandle::parse("".to_string()));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_handle_parsed_successfully(handle: ValidHandleFixture) -> bool {
        UserHandle::parse(handle.0).is_ok()
    }
}
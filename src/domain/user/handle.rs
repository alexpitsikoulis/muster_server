#[derive(Debug)]
pub enum HandleValidationErr {
    HandleEmpty,
    HandleTooLong,
    HandleContainsWhiteSpace,
    HandleContainsForbiddenChars(char),
}

#[derive(Debug)]
pub struct UserHandle(String);

impl UserHandle {
    pub fn parse(handle: String) -> Result<Self, HandleValidationErr> {
        let forbidden_characters = ['/', '(', ')', '"', '\'', '<', '>', '\\', '{', '}'];
        let mut forbidden_character: Option<char> = None;
        if handle.trim().is_empty() {
            return Err(HandleValidationErr::HandleEmpty)
        } else if handle.len() > 20 {
            return Err(HandleValidationErr::HandleTooLong)
        } else if handle
                .chars()
                .any(|c| {
                    let contains_forbidden = forbidden_characters.contains(&c) || c.is_whitespace();

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
    use crate::domain::user::*;
    use claim::{assert_err, assert_ok};

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
    fn empty_string_handle_rejected() {
        let handles = &["/", "(", ")", "'", "\"", "<", ">", "\\", "{", "}"];
        for handle in handles {
            assert_err!(UserHandle::parse(handle.to_string()));
        }
    }

    #[test]
    fn valid_handle_parsed_successfully() {
        let handles = &[
            "alexpitsikoulis",
            "alex.pitsikoulis",
            "AlexPitsikoulis",
            "ALEX--PITSIKOULIS",
        ];
        for handle in handles {
            assert_ok!(UserHandle::parse(handle.to_string()));
        }
    }
}
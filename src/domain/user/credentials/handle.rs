use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug)]
pub enum HandleValidationErr {
    HandleEmpty,
    HandleTooLong,
    HandleContainsWhiteSpace,
    HandleContainsForbiddenChars(char),
}

pub const ALLOWED_HANDLE_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '#', '$', '%', '&', '*', '+', ',', '-', '.', ':', ';', '=', '?',
    '@', '[', ']', '^', '_', '`', '|', '~',
];

#[derive(Debug, Clone, PartialEq)]
pub struct Handle(String);

impl std::fmt::Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(HandleVisitor)
    }
}

struct HandleVisitor;

impl<'de> Visitor<'de> for HandleVisitor {
    type Value = Handle;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a handle no longer than 20 characters containing only letters, numbers, or the following characters:  '!', '#', '$', '%', '&', '*', '+', ',', '-', '.', ':', ';', '=', '?',
        '@', '[', ']', '^', '_', '`', '|', '~'")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Handle::try_from(v.as_str()).map_err(|_| E::invalid_value(Unexpected::Str(&v), &self))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }
}

impl AsRef<str> for Handle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Handle {
    type Error = HandleValidationErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut forbidden_character: Option<char> = None;
        if value.trim().is_empty() {
            return Err(HandleValidationErr::HandleEmpty);
        } else if value.len() > 20 {
            return Err(HandleValidationErr::HandleTooLong);
        } else if value.chars().any(|c| {
            let contains_forbidden = !ALLOWED_HANDLE_CHARS.contains(&c);

            if contains_forbidden {
                forbidden_character = Some(c);
            }

            contains_forbidden
        }) {
            return Err(HandleValidationErr::HandleContainsForbiddenChars(
                forbidden_character.unwrap(),
            ));
        }
        Ok(Handle(value))
    }
}

impl TryFrom<&str> for Handle {
    type Error = HandleValidationErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::user::*, utils::test::HANDLE_GENERATOR};
    use claim::{assert_err, assert_ok};

    #[derive(Clone, Debug)]
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
        assert_ok!(Handle::try_from(handle));
    }

    #[test]
    fn a_handle_longer_than_20_grapheme_is_rejected() {
        let handle = "A".repeat(21);
        assert_err!(Handle::try_from(handle));
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
            assert_err!(Handle::try_from(handle.to_string()));
        }
    }

    #[test]
    fn handle_with_forbidden_characters_rejected() {
        let handles = &["/", "(", ")", "'", "\"", "<", ">", "\\", "{", "}"];
        for handle in handles {
            assert_err!(Handle::try_from(handle.to_string()));
        }
    }

    #[test]
    fn empty_string_handle_rejected() {
        assert_err!(Handle::try_from("".to_string()));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_handle_parsed_successfully(handle: ValidHandleFixture) -> bool {
        Handle::try_from(handle.0).is_ok()
    }
}

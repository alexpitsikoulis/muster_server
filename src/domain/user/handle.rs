#[derive(Debug)]
pub enum HandleValidationErr {
    HandleEmpty,
    HandleTooLong,
    HandleContainsWhiteSpace,
    HandleContainsForbiddenChars(char),
}
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
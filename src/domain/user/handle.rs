#[derive(Debug)]
pub enum HandleValidationErr {
    HandleEmpty,
    HandleTooLong,
    HandleContainsWhiteSpace,
    HandleContainsForbiddenChars(char),
}
pub struct UserHandle(String);

impl UserHandle {
    pub fn parse(handle: String) -> Self  {
        let has_whitespace = handle.split_whitespace().count() != 1;
        
        let is_empty = handle.trim().is_empty();
        
        let is_too_long = handle.len() > 20;

        let forbidden_characters = ['/', '(', ')', '"', '\'', '<', '>', '\\', '{', '}'];
        let has_forbidden_character = handle
            .chars()
            .any(|c| forbidden_characters.contains(&c));

        if has_whitespace || is_empty || is_too_long || has_forbidden_character {
            panic!("{} is not a valid user handle", handle);
        }

        UserHandle(handle)
    }

    pub fn try_parse(handle: String) -> Result<Self, HandleValidationErr> {
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

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
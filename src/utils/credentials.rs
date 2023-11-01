use rand::RngCore;
use argon2::{self, Config};
use secrecy::{Secret, ExposeSecret};

#[derive(Debug)]
pub enum CredentialValidationError {
    HandleEmpty,
    HandleTooLong,
    HandleContainsForbiddenChars(char),
    PwdTooShort,
    PwdTooLong,
    PwdMissingLowercase,
    PwdMissingUppercase,
    PwdMissingNumber,
    PwdMissingChar,
    ArgonErr(argon2::Error),
}

type Result<T> = std::result::Result<T, CredentialValidationError>;

pub fn is_valid_handle(handle: String) -> Result<()> {
    let forbidden_characters = ['/', '(', ')', '"', '\'', '<', '>', '\\', '{', '}'];
    let mut forbidden_character: Option<char> = None;
    if handle.trim().is_empty() {
        return Err(CredentialValidationError::HandleEmpty)
    } else if handle.len() > 20 {
        return Err(CredentialValidationError::HandleTooLong)
    } else if handle
        .chars()
        .any(|c| {
            forbidden_character = Some(c);
            forbidden_characters.contains(&c)
        }) {
            return Err(CredentialValidationError::HandleContainsForbiddenChars(forbidden_character.unwrap()))
    }
    Ok(())
}


pub fn validate_and_hash_password(password: Secret<String>) -> Result<String> {
    match validate_password(password.clone()) {
        Ok(()) => {
            let salt = {
                let mut unencoded = [0u8; 16];
                let mut rng = rand::thread_rng();
                rng.fill_bytes(&mut unencoded);
                unencoded
            };
        
            let config = Config::original();
            
            match argon2::hash_encoded(password.expose_secret().as_bytes(), &salt, &config) {
                Ok(hash) => Ok(hash),
                Err(e) => Err(CredentialValidationError::ArgonErr(e)),
            }
        },
        Err(e) => Err(e),
    } 
}

pub fn compare_password_hash(password: Secret<String>, hash: String) -> bool {
    argon2::verify_encoded(&hash, password.expose_secret().as_bytes()).unwrap()
}

pub fn validate_password(password: Secret<String>) -> Result<()> {
    if password.expose_secret().len() < 8 {
        return Err(CredentialValidationError::PwdTooShort);
    }

    if password.expose_secret().len() > 64 {
       return Err(CredentialValidationError::PwdTooLong);
    }

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_number = false;
    let mut has_char = false;

    for b in password.expose_secret().as_bytes() {
        if has_lower && has_upper && has_number && has_char {
            break;
        }
        if !has_lower && *b >= 97 && *b <= 122 {
            has_lower = true;
            continue;
        }
        if !has_upper && *b >= 65 && *b <= 90 {
            has_upper = true;
            continue;
        }
        if !has_number && *b >= 48 && *b <= 57 {
            has_number = true;
            continue;
        }
        if !has_char && " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes().contains(b) {
            has_char = true;
            continue;
        }
    };

    if !has_lower {
        return Err(CredentialValidationError::PwdMissingLowercase);
    }
    if !has_upper {
        return Err(CredentialValidationError::PwdMissingUppercase);
    }
    if !has_number {
        return Err(CredentialValidationError::PwdMissingNumber);
    }
    if !has_char {
        return Err(CredentialValidationError::PwdMissingChar);
    }

    Result::Ok(())
}
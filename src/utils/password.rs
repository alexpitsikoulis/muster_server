use std::env;

use rand::RngCore;
use argon2::{self, Config};
use secrecy::{Secret, ExposeSecret};

use crate::config::Env;

#[derive(Debug)]
pub enum PasswordValidationError {
    PwdTooShort,
    PwdTooLong,
    PwdMissingLowercase,
    PwdMissingUppercase,
    PwdMissingNumber,
    PwdMissingChar,
    ArgonErr(argon2::Error),
}

type Result<T> = std::result::Result<T, PasswordValidationError>;

pub fn validate_and_hash_password(password: Secret<String>) -> Result<String> {
    match validate_password(password.clone()) {
        Ok(()) => {
            let salt = {
                let mut unencoded = [0u8; 16];
                let mut rng = rand::thread_rng();
                rng.fill_bytes(&mut unencoded);
                unencoded
            };
        
            let config = if std::env::var("APP_ENVIRONMENT").is_ok_and(|env| Env::from(env) == Env::Test) {
                Config::original()
            } else {
                Config::default()
            };
            
            match argon2::hash_encoded(password.expose_secret().as_bytes(), &salt, &config) {
                Ok(hash) => Ok(hash),
                Err(e) => Err(PasswordValidationError::ArgonErr(e)),
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
        return Err(PasswordValidationError::PwdTooShort);
    }

    if password.expose_secret().len() > 64 {
        return Err(PasswordValidationError::PwdTooLong);
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
        return Err(PasswordValidationError::PwdMissingLowercase);
    }
    if !has_upper {
        return Err(PasswordValidationError::PwdMissingUppercase);
    }
    if !has_number {
        return Err(PasswordValidationError::PwdMissingNumber);
    }
    if !has_char {
        return Err(PasswordValidationError::PwdMissingChar);
    }

    Result::Ok(())
}
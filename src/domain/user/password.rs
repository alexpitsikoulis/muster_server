use rand::RngCore;
use argon2::{self, Config};
use secrecy::{Secret, ExposeSecret};

#[derive(Debug)]
pub enum PasswordValidationErr {
    PwdTooShort,
    PwdTooLong,
    PwdMissingLowercase,
    PwdMissingUppercase,
    PwdMissingNumber,
    PwdMissingChar,
    ArgonErr(argon2::Error),
}

pub struct UserPassword(String);

impl UserPassword {
    pub fn parse(password: Secret<String>) -> Self {
        match Self::validate(password.clone()) {
            Ok(()) => {
                let salt = {
                    let mut unencoded = [0u8; 16];
                    let mut rng = rand::thread_rng();
                    rng.fill_bytes(&mut unencoded);
                    unencoded
                };
            
                let config = Config::original();
                
                match argon2::hash_encoded(password.expose_secret().as_bytes(), &salt, &config) {
                    Ok(hash) => Self(hash),
                    Err(e) => panic!("Password hash failed: {:?}", e),
                }
            },
            Err(e) => panic!("Password is invalid: {:?}", e),
        }
    }

    pub fn try_parse(password: Secret<String>) -> Result<Self, PasswordValidationErr> {
        match Self::validate(password.clone()) {
            Ok(()) => {
                let salt = {
                    let mut unencoded = [0u8; 16];
                    let mut rng = rand::thread_rng();
                    rng.fill_bytes(&mut unencoded);
                    unencoded
                };
            
                let config = Config::original();
                
                match argon2::hash_encoded(password.expose_secret().as_bytes(), &salt, &config) {
                    Ok(hash) => Ok(Self(hash)),
                    Err(e) => Err(PasswordValidationErr::ArgonErr(e)),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn validate(password: Secret<String>) -> Result<(), PasswordValidationErr> {
        if password.expose_secret().len() < 8 {
            return Err(PasswordValidationErr::PwdTooShort);
        }
    
        if password.expose_secret().len() > 64 {
           return Err(PasswordValidationErr::PwdTooLong);
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
            return Err(PasswordValidationErr::PwdMissingLowercase);
        }
        if !has_upper {
            return Err(PasswordValidationErr::PwdMissingUppercase);
        }
        if !has_number {
            return Err(PasswordValidationErr::PwdMissingNumber);
        }
        if !has_char {
            return Err(PasswordValidationErr::PwdMissingChar);
        }
    
        Result::Ok(())
    }

    pub fn compare(password: Secret<String>, hash: String) -> bool {
        argon2::verify_encoded(&hash, password.expose_secret().as_bytes()).unwrap()
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
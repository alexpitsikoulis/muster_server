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

#[derive(Debug)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn parse(password: Secret<String>) -> Result<Self, PasswordValidationErr> {
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
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::UserPassword;
    use claim::{assert_err, assert_ok};
    use secrecy::Secret;

    #[test]
    fn fails_when_less_than_8_grapheme() {
        let password = Secret::new("P@ssw0r".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn fails_when_more_than_64_grapheme() {
        let filler = "A".repeat(60);
        let password = Secret::new(format!("P@ss1{}", filler).to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn fails_when_no_uppercase() {
        let password = Secret::new("n0neofyourbus!ness".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn fails_when_no_lowercase() {
        let password = Secret::new("N0NEOFYOURBUS!NESS".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn fails_when_no_number() {
        let password = Secret::new("Noneofyourbus!ness".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn fails_when_no_special_char() {
        let password = Secret::new("N0neofyourbusiness".to_string());
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn valid_password_parses_successfully() {
        let passwords = &[
            "N0neofyourbus!ness",
            "N0neofyourbusiness\"",
            "N0neofyourbusiness#",
            "N0neofyourbusiness$",
            "N0neofyourbusiness%",
            "N0neofyourbusiness&",
            "N0neofyourbusiness'",
            "N0neofyourbusiness(",
            "N0neofyourbusiness)",
            "N0neofyourbusiness*",
            "N0neofyourbusiness+",
            "N0neofyourbusiness-",
            "N0neofyourbusiness.",
            "N0neofyourbusiness/",
            "N0neofyourbusiness:",
            "N0neofyourbusiness;",
            "N0neofyourbusiness<",
            "N0neofyourbusiness=",
            "N0neofyourbusiness>",
            "N0neofyourbusiness?",
            "N0neofyourbusiness@",
            "N0neofyourbusiness[",
            "N0neofyourbusiness]",
            "N0neofyourbusiness\\",
            "N0neofyourbusiness^",
            "N0neofyourbusiness_",
            "N0neofyourbusiness`",
            "N0neofyourbusiness{",
            "N0neofyourbusiness|",
            "N0neofyourbusiness}",
            "N0neofyourbusiness~",
        ];
        for password in passwords {
            assert_ok!(UserPassword::parse(Secret::new(password.to_string())));
        }
    }
}
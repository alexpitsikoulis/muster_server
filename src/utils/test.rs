use crate::domain::user::{ALLOWED_HANDLE_CHARS, ALLOWED_PASSWORD_CHARS};
use quickcheck::Gen;

const LOWERCASE: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const UPPERCASE: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub enum RandomStringGeneratorError {
    ErrMinLowerThanRequired,
    ErrMaxLowerThanMin,
}

pub struct RandomStringGenerator {
    min_length: i32,
    max_length: i32,
    req_lowercase: bool,
    req_uppercase: bool,
    req_number: bool,
    req_symbol: bool,
    allowed_chars: &'static [char],
}

impl RandomStringGenerator {
    pub fn new(
        min_length: i32,
        max_length: i32,
        req_lowercase: bool,
        req_uppercase: bool,
        req_number: bool,
        req_symbol: bool,
        allowed_chars: &'static [char],
    ) -> Result<Self, RandomStringGeneratorError> {
        if max_length < min_length {
            return Err(RandomStringGeneratorError::ErrMaxLowerThanMin);
        }
        let mut req_min = 0;
        if req_lowercase {
            req_min += 1;
        }
        if req_uppercase {
            req_min += 1;
        }
        if req_number {
            req_min += 1;
        }
        if req_symbol {
            req_min += 1;
        }
        if req_min > min_length {
            return Err(RandomStringGeneratorError::ErrMinLowerThanRequired);
        }
        Ok(RandomStringGenerator {
            min_length,
            max_length,
            req_lowercase,
            req_uppercase,
            req_number,
            req_symbol,
            allowed_chars,
        })
    }

    pub fn generate(&self, rng: &mut Gen) -> String {
        let mut inserted = 0;
        let mut s = String::new();

        if self.req_lowercase {
            s.push(*rng.choose(&LOWERCASE).unwrap());
            inserted += 1;
        };
        if self.req_uppercase {
            s.push(*rng.choose(&UPPERCASE).unwrap());
            inserted += 1;
        };
        if self.req_number {
            s.push(*rng.choose(&NUMBERS).unwrap());
            inserted += 1;
        };
        if self.req_symbol {
            let allowed_chars_length = self.allowed_chars.len() as i32;
            let char_index = gen_range(0, allowed_chars_length - 1, rng) as usize;
            s.push(self.allowed_chars[char_index]);
            inserted += 1;
        };

        let charset = &[LOWERCASE, UPPERCASE, NUMBERS, self.allowed_chars].concat();
        for _ in 0..gen_range(self.min_length - inserted, self.max_length - inserted, rng) {
            s.push(*rng.choose(charset).unwrap());
        }

        s
    }
}

pub const HANDLE_GENERATOR: RandomStringGenerator = RandomStringGenerator {
    min_length: 1,
    max_length: 20,
    req_lowercase: false,
    req_uppercase: false,
    req_number: false,
    req_symbol: false,
    allowed_chars: ALLOWED_HANDLE_CHARS,
};

pub const PASSWORD_GENERATOR: RandomStringGenerator = RandomStringGenerator {
    min_length: 8,
    max_length: 64,
    req_lowercase: true,
    req_uppercase: true,
    req_number: true,
    req_symbol: true,
    allowed_chars: ALLOWED_PASSWORD_CHARS,
};

pub fn gen_range(min: i32, max: i32, rng: &mut Gen) -> i32 {
    let length_range = Vec::<i32>::from_iter(min..=max);
    let length = rng.choose(length_range.as_slice()).unwrap();
    return *length;
}

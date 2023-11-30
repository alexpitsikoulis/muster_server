#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{domain::confirmation_token::ConfirmationToken, utils::jwt::generate_token};
    use secrecy::Secret;
    use uuid::Uuid;

    #[derive(Clone, Debug)]
    struct UserIdFixture(pub String);

    impl quickcheck::Arbitrary for UserIdFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let id = Uuid::new_v4();
            UserIdFixture(id.to_string())
        }
    }

    #[quickcheck_macros::quickcheck]
    fn token_successfully_generated_on_valid_user_id(user_id: UserIdFixture) -> bool {
        let user_id = user_id.0;
        let id = Uuid::from_str(&user_id).expect("Failed to parse user id from test fixture");
        let confirmation_token =
            Secret::new(generate_token(id).expect("Failed to generate confirmation token"));
        ConfirmationToken::new(confirmation_token, Uuid::from_str(&user_id).unwrap());
        true
    }
}

use chrono::Utc;
use muttr_server::{
    domain::user::{Email, Handle, Password, User},
    storage::upsert_user,
};
use secrecy::Secret;
use uuid::Uuid;

pub const TEST_USER_PASSWORD: &str = "Testpassw0rd!";

use super::TestDB;

impl TestDB {
    pub async fn insert_user(&mut self, email: &str, handle: &str, email_confirmed: bool) -> User {
        let now = Utc::now();
        let mut user = User::new(
            Uuid::new_v4(),
            Email::parse_str(email).expect(&format!("Email '{}' is invalid", email)),
            Handle::parse_str(handle).expect(&format!("Handle '{}' is invalid", handle)),
            None,
            TEST_USER_PASSWORD.into(),
            None,
            None,
            0,
            email_confirmed,
            now,
            now,
            None,
        );

        user.set_password(match Password::parse(Secret::new(user.password())) {
            Ok(password) => password,
            Err(e) => panic!("Password validation failed: {:?}", e),
        });

        match upsert_user(&self.db_pool, &user).await {
            Ok(_) => user,
            Err(e) => panic!("Failed to insert user: {:?}", e),
        }
    }
}

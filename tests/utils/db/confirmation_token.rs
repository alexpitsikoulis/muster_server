use muttr_server::{
    domain::confirmation_token::ConfirmationToken, storage::insert_confirmation_token,
    utils::jwt::generate_token,
};
use secrecy::Secret;
use uuid::Uuid;

use super::TestDB;

impl TestDB {
    pub async fn insert_confirmation_token(&mut self, user_id: Uuid) -> ConfirmationToken {
        let token = generate_token(user_id).expect("Failed to generate test confirmation token");
        let confirmation_token = ConfirmationToken::new(Secret::new(token), user_id);
        match insert_confirmation_token(&self.db_pool, &confirmation_token).await {
            Ok(_) => confirmation_token,
            Err(e) => panic!("Failed to insert test confirmation token: {:?}", e),
        }
    }
}

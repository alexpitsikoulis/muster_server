use chrono::Utc;
use muttr_server::{
    domain::server::Server,
    storage::{get_server_by_id, upsert_server},
};
use uuid::Uuid;

use super::TestDB;

impl TestDB {
    pub async fn insert_server(&mut self, owner_id: Uuid) -> Server {
        let now = Utc::now();
        let server = Server::new(
            Uuid::new_v4(),
            "Test Server".to_string(),
            owner_id,
            Some("Just a test server".to_string()),
            None,
            None,
            now,
            now,
            None,
        );

        match upsert_server(&self.db_pool, &server).await {
            Ok(_) => server,
            Err(e) => panic!("Failed to insert server: {:?}", e),
        }
    }

    pub async fn get_server_by_id(&mut self, id: Uuid) -> Server {
        get_server_by_id(&self.db_pool, id).await.unwrap()
    }
}

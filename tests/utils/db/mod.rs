mod confirmation_token;
pub mod server;
pub mod user;

use muttr_server::{config::DatabaseConfig, startup::App};
use sqlx::{Connection, Executor, PgConnection, PgPool};

pub struct TestDB {
    pub config: DatabaseConfig,
    pub db_pool: PgPool,
}

impl TestDB {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let db_pool = App::get_connection_pool(config);
        let mut test_db = TestDB {
            config: config.clone(),
            db_pool,
        };
        test_db.configure().await;
        test_db
    }

    pub async fn configure(&mut self) {
        let mut connection = PgConnection::connect_with(&self.config.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, self.config.database_name).as_str())
            .await
            .expect("Failed to create database");

        sqlx::migrate!("./migrations")
            .run(&self.db_pool)
            .await
            .expect("Failed to migrate database");
    }

    pub async fn clear(&mut self, table_name: &str) {
        self.db_pool
            .execute(format!(r#"DELETE FROM {}"#, table_name).as_str())
            .await
            .expect(format!("Failed to clear {} table in database", table_name).as_str());
    }
}

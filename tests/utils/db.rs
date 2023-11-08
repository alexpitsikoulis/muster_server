use chrono::Utc;
use secrecy::Secret;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use muttr_server::{
    config::DatabaseConfig,
    startup::App,
    storage::upsert_user,
    domain::user::{User, Password},
};
use super::{TEST_USER_EMAIL, TEST_USER_HANDLE, TEST_USER_PASSWORD};

pub struct TestDB {
    pub config: DatabaseConfig,
    pub db_pool: PgPool,
}


impl TestDB {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let db_pool = App::get_connection_pool(config);
        let mut test_db = TestDB { config: config.clone(), db_pool };
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

    pub async fn insert_user(&mut self, email_confirmed: bool) -> User {
        let now = Utc::now();
        let mut user = User::new(
            Uuid::new_v4(),
            TEST_USER_EMAIL.into(),
            TEST_USER_HANDLE.into(),
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
    
    pub async fn clear(&mut self, table_name: &str) {
        self.db_pool.execute(format!(r#"DELETE FROM {}"#, table_name).as_str())
            .await
            .expect(format!("Failed to clear {} table in database", table_name).as_str());
    }
}
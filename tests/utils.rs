use std::net::TcpListener;
use chrono::Utc;
use muttr_server::config::{Config, DatabaseConfig};
use muttr_server::storage::{User, upsert_user};
use muttr_server::utils::validate_and_hash_password;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;

pub struct TestApp {
    pub config: Config,
    pub address: String,
    pub db_pool: PgPool,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.db_pool.execute(format!(r#"DROP DATABASE "{}";"#, self.config.database.database_name).as_str());
    }
}

pub async fn spawn_app() -> TestApp {
    let mut config = muttr_server::config::get_config(Some("tests/config.yaml"))
        .expect("Failed to load test config file");
    config.database.database_name = Uuid::new_v4().to_string();

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let connection_pool = configure_database(&config.database).await;
    let server = muttr_server::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        config,
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseConfig) -> PgPool {
    let mut connection = PgConnection::connect(
            &config.test_connection_string(),
        )
        .await
        .expect("Failed to connect to Postgres");
    
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

pub async fn insert_user(db_pool: &PgPool, user: Option<User>) -> User {
    let now = Utc::now();
    let mut inserted_user = match user {
        Some(u) => u,
        None => {
            User::new(
                Uuid::new_v4(),
                "testuser@youwish.com".into(),
                "Test User".into(),
                "Testpassw0rd!".into(),
                None,
                None,
                0,
                now,
                now,
                None,
            )
         }
    };
    inserted_user.password = match validate_and_hash_password(inserted_user.password) {
        Ok(hash) => hash,
        Err(e) => panic!("Password validation failed: {:?}", e),
    };
    
    match upsert_user(db_pool, &inserted_user).await {
        Ok(_) => inserted_user,
        Err(e) => panic!("Failed to insert user: {:?}", e),
    }
}

pub async fn clear_database(db_pool: &PgPool, table_name: &str) {
    db_pool.execute(format!(r#"DELETE FROM {}"#, table_name).as_str())
        .await
        .expect(format!("Failed to clear {} table in database", table_name).as_str());
}
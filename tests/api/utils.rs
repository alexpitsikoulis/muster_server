use chrono::Utc;
use muttr_server::{
    domain::user::UserPassword,
    config::{Config, DatabaseConfig, get_config},
    utils::telemetry::{create_subscriber, init_subscriber},
    storage::{User, upsert_user}, startup::App,
};
use secrecy::Secret;
use sqlx::{PgPool, PgConnection, Executor, Connection};
use uuid::Uuid;
use once_cell::sync::Lazy;

const TEST_USER_EMAIL: &str = "testuser@youwish.com";
const TEST_USER_HANDLE: &str = "test.user";
const TEST_USER_PASSWORD: &str = "Testpassw0rd!"; 

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "test".to_string();
    let env_filter = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = create_subscriber(name, env_filter, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = create_subscriber(name, env_filter, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub config: Config,
    pub address: String,
    pub database: TestDB,
    pub client: reqwest::Client,
}

pub struct TestDB {
    pub config: DatabaseConfig,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn spawn() -> Self {
        std::env::set_var("APP_ENVIRONMENT", "test");
        Lazy::force(&TRACING);
        
        let config = {
            let mut c = get_config().expect("Failed to load test config file");
            c.database.database_name = Uuid::new_v4().to_string();
            c.app.port = 0;
            c
        };
        
        let app = App::build(config.clone())
            .await
            .expect("Failed to build app");
        let address = format!("http://127.0.0.1:{}", app.port());
        let _ = tokio::spawn(app.run_until_stopped());
    
        let test_db = TestDB::new(&config.database).await;
        TestApp {
            config: config.clone(),
            address: address,
            database: test_db,
            client: reqwest::Client::new(),
        }
    }
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
    
        user.password = match UserPassword::parse(Secret::new(user.password)) {
            Ok(hash) => hash.as_ref().to_string(),
            Err(e) => panic!("Password validation failed: {:?}", e),
        };
        
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
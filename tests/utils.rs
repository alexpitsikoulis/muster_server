use chrono::Utc;
use muttr_server::{
    domain::user::UserPassword,
    config::{Config, DatabaseConfig},
    utils::{create_subscriber, init_subscriber},
    storage::{User, upsert_user},
};
use secrecy::Secret;
use std::net::TcpListener;
use sqlx::{PgPool, PgConnection, Executor, Connection};
use uuid::Uuid;
use once_cell::sync::Lazy;

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
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    std::env::set_var("APP_ENVIRONMENT", "test");
    Lazy::force(&TRACING);
    
    let mut config = muttr_server::config::get_config()
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
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

#[allow(dead_code)]
pub async fn insert_user(db_pool: &PgPool, user: Option<User>) -> User {
    let now = Utc::now();
    let mut inserted_user = match user {
        Some(u) => u,
        None => {
            User::new(
                Uuid::new_v4(),
                "testuser@youwish.com".into(),
                "alex.pitsikoulis".into(),
                None,
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
    inserted_user.password = match UserPassword::try_parse(Secret::new(inserted_user.password)) {
        Ok(hash) => hash.inner_ref().to_string(),
        Err(e) => panic!("Password validation failed: {:?}", e),
    };
    
    match upsert_user(db_pool, &inserted_user).await {
        Ok(_) => inserted_user,
        Err(e) => panic!("Failed to insert user: {:?}", e),
    }
}

#[allow(dead_code)]
pub async fn clear_database(db_pool: &PgPool, table_name: &str) {
    db_pool.execute(format!(r#"DELETE FROM {}"#, table_name).as_str())
        .await
        .expect(format!("Failed to clear {} table in database", table_name).as_str());
}
use muttr_server::{
    config::{Config, get_config},
    utils::telemetry::{create_subscriber, init_subscriber},
    startup::App,
};
use uuid::Uuid;
use once_cell::sync::Lazy;
use wiremock::MockServer;
use super::{
    db::TestDB,
    http_client::Client,
};

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
    pub client: Client,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn spawn() -> Self {
        std::env::set_var("APP_ENVIRONMENT", "test");
        Lazy::force(&TRACING);
        let email_server = MockServer::start().await;
        
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
            address: address.clone(),
            database: test_db,
            client: Client::new(address.clone()),
            email_server: email_server,
        }
    }
}


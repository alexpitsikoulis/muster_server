use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use muttr_server::{
    startup::run,
    config::get_config,
    utils::{create_subscriber, init_subscriber}
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    let subscriber = create_subscriber(
        subscriber_name, default_filter_level, std::io::stdout
    );
    init_subscriber(subscriber);
    let config = get_config(Some("config.yaml")).expect("Failed to read config file");
    let connection_pool = PgPool::connect(
        &config.database.connection_string().expose_secret()
        )
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to port 8000");
    run(listener, connection_pool)?.await
}
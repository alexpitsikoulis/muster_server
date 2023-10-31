use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
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
    let config = get_config().expect("Failed to read config file");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());
    
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to port 8000");
    run(listener, connection_pool)?.await
}
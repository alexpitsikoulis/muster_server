use std::net::TcpListener;

use muttr_server::{startup::run, config::get_config};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config(Some("config.yaml")).expect("Failed to read config file");
    let connection_pool = PgPool::connect(
        &config.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to port 8000");
    run(listener, connection_pool)?.await
}
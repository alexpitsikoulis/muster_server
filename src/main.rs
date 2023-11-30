use muttr_server::{
    config::get_config,
    startup::App,
    utils::telemetry::{create_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = create_subscriber("muttr_server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config file");
    App::build(config).await?.run_until_stopped().await
}

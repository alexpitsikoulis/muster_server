use secrecy::{ExposeSecret, Secret};
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(PartialEq)]
pub enum Env {
    Local,
    Production,
    Test,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Local => "local",
            Env::Production => "production",
            Env::Test => "test",
        }
    }
}

impl From<String> for Env {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "local" => Self::Local,
            "production" => Self::Production,
            "test" => Self::Test,
            other => {
                tracing::warn!(
                    "{} is not a supported environment. \
                    Use either `local`, `production`, or `test`",
                    other
                );
                Self::Local
            }
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub email_client: EmailClientConfig,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailClientConfig {
    pub base_url: String,
    pub sender_email: String,
}

impl DatabaseConfig {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_directory = base_path.join("config");
    let env: Env = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .into();
    let env_filename = format!("{}.yaml", env.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_directory.join("base.yaml")))
        .add_source(config::File::from(config_directory.join(env_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Config>()
}

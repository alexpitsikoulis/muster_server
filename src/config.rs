use secrecy::{Secret, ExposeSecret};

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
                    Use either `local` or `production`",
                    other
                );
                Self::Local
            },
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(serde::Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database_name: String
}


impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name,
        ))
    }

    pub fn test_connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port,
        ))
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let config_directory = base_path.join("config");
    let env: Env = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .into();
    let env_filename = format!("{}.yaml", env.as_str());
    
    let settings = config::Config::builder()
        .add_source(
            config::File::from(config_directory.join("base.yaml"))
        )
        .add_source(
            config::File::from(config_directory.join(&env_filename))
        )
        .build()?;

    settings.try_deserialize::<Config>()
}
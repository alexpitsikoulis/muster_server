#[derive(serde::Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name,
        )
    }

    pub fn test_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }
}

pub fn get_config(config_file: Option<&'static str>) -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new(match config_file {
                Some(path) => path,
                None => "config.yaml"
            }, config::FileFormat::Yaml)
        )
        .build()?;

    settings.try_deserialize::<Config>()
}
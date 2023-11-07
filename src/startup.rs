use lettre::transport::smtp::authentication::Credentials;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::TcpListener, sync::Arc};
use tracing_actix_web::TracingLogger;
use actix_web::{
    HttpServer,
    dev::Server,
    web::{get, post, Data},
};
use crate::{
    config::{Config, DatabaseConfig, MailerConfig},
    handlers::{
        health_check,
        signup,
        login,
        create_server,
    }, domain::mailer::Mailer,
};

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let db_pool = Self::get_connection_pool(&config.database);
        
        let address = format!(
            "{}:{}",
            config.app.host, config.app.port,
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = Self::run(listener, db_pool, config)?;
        
        Ok(Self{ port, server })
    }
    
    
    fn run(listener: TcpListener, db_pool: PgPool, config: Config) -> Result<Server, std::io::Error> {
        let db_pool = Data::new(db_pool);
        let server = HttpServer::new(move || {
            let mailer = Self::init_mailer(config.mailer.clone());
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .route("/health-check", get().to(health_check))
                .route("/signup", post().to(signup))
                .route("/login", post().to(login))
                .route("/servers", post().to(create_server))
                .app_data(db_pool.clone())
                .app_data(Arc::new(mailer))
            })
                .listen(listener)?
                .run();
        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn get_connection_pool(config: &DatabaseConfig) -> PgPool {
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(config.with_db())
    }

    fn init_mailer(config: MailerConfig) -> Mailer {
        Mailer::new(
            format!("{}:{}", config.host, config.port),
            Credentials::new(
                config.username,
                config.password
                    .expose_secret()
                    .clone(),
            )
        ).expect("Failed to init email client")
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

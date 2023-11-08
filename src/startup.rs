use lettre::transport::smtp::authentication::Credentials;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{net::TcpListener, sync::Arc};
use tracing_actix_web::TracingLogger;
use actix_web::{
    HttpServer,
    dev::Server,
    web::{get, post, Data},
};
use crate::{
    domain::{mailer::Mailer, user::Email},
    config::{Config, DatabaseConfig, MailerConfig},
    handlers::{
        health_check::health_check,
        user,
        server,
    },
};

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            config.app.host, config.app.port,
        );
        
        let db_pool = Self::get_connection_pool(&config.database);

        let sender_email = match Email::parse(config.mailer.sender_email) {
            Ok(email) => email,
            Err(e) => {
                tracing::error!("Failed to parse mailer sender_email from config: {:?}", e);
                panic!("Failed to parse mailer sender_email from config: {:?}", e)
            },
        };

        let mailer = Mailer::new(address.clone(), sender_email);
        
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = Self::run(listener, db_pool, mailer)?;
        
        Ok(Self{ port, server })
    }
    
    
    fn run(listener: TcpListener, db_pool: PgPool, mailer: Mailer) -> Result<Server, std::io::Error> {
        let db_pool = Data::new(db_pool);
        let mailer = Data::new(mailer);
        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .route("/health-check", get().to(health_check))
                .route("/signup", post().to(user::signup))
                .route("/login", post().to(user::login))
                .route("/servers", post().to(server::create_server))
                .app_data(db_pool.clone())
                .app_data(mailer.clone())
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

    pub fn port(&self) -> u16 {
        self.port
    }
}

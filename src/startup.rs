use crate::handlers::{
    health_check,
    signup,
    login,
    create_server,
};
use actix_web::{HttpServer, App};
use actix_web::web::{get, post, Data};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", get().to(health_check))
            .route("/signup", post().to(signup))
            .route("/login", post().to(login))
            .route("/servers", post().to(create_server))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
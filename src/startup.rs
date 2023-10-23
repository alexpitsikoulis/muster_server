use crate::handlers::{health_check, signup, login};
use actix_web::{HttpServer, App};
use actix_web::web::{get, post, Data};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", get().to(health_check))
            .route("/signup", post().to(signup))
            .route("/login", post().to(login))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
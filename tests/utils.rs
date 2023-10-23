use std::net::TcpListener;
use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let config = muttr_server::config::get_config(Some("tests/config.yaml"))
        .expect("Failed to load test config file");
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let connection_pool = PgPool::connect(
            &config.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres");
    let server = muttr_server::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn clear_database(db_pool: &PgPool) {
    sqlx::query!("DELETE FROM users")
        .execute(db_pool)
        .await
        .expect("Failed to clear users table in database");
    sqlx::query!("DELETE FROM servers")
        .execute(db_pool)
        .await
        .expect("Failed to clear servers table in database");
    sqlx::query!("DELETE FROM server_members")
        .execute(db_pool)
        .await
        .expect("Failed to clear server_members table in database");
    sqlx::query!("DELETE FROM group_chats")
        .execute(db_pool)
        .await
        .expect("Failed to clear group_chats table in database");
    sqlx::query!("DELETE FROM posts")
        .execute(db_pool)
        .await
        .expect("Failed to clear posts table in database");
    sqlx::query!("DELETE FROM post_likes")
        .execute(db_pool)
        .await
        .expect("Failed to clear post_likes table in database");
    sqlx::query!("DELETE FROM comments")
        .execute(db_pool)
        .await
        .expect("Failed to clear comments table in database");
    sqlx::query!("DELETE FROM comment_likes")
        .execute(db_pool)
        .await
        .expect("Failed to clear comment_likes table in database");
}
use crate::domain::server::Server;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool};
use uuid::Uuid;

pub const SERVERS_TABLE_NAME: &str = "servers";

#[tracing::instrument(
    name = "Upserting server details to database",
    skip(server, db_pool),
    fields(
        server_data = %server,
    )
)]
pub async fn upsert_server(db_pool: &PgPool, server: &Server) -> Result<PgQueryResult, Error> {
    query(
        r#"
        INSERT INTO servers (id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (id)
        DO
            UPDATE SET
                name = EXCLUDED.name,
                owner_id = EXCLUDED.owner_id,
                description = EXCLUDED.description,
                photo = EXCLUDED.photo,
                cover_photo = EXCLUDED.cover_photo,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at
        WHERE
            (servers.name, servers.owner_id, servers.description, servers.photo, servers.cover_photo, servers.deleted_at) IS DISTINCT FROM
            (EXCLUDED.name, EXCLUDED.owner_id, EXCLUDED.description, EXCLUDED.photo, EXCLUDED.cover_photo, EXCLUDED.deleted_at);

        "#)
        .bind(server.id())
        .bind(server.name())
        .bind(server.owner_id())
        .bind(server.description())
        .bind(server.photo())
        .bind(server.cover_photo())
        .bind(server.created_at())
        .bind(server.updated_at())
        .bind(server.deleted_at())
    .execute(db_pool)
    .await
}

#[tracing::instrument(
    name = "Getting server by id",
    skip(id, db_pool),
    fields(
        server_id = %id
    )
)]
pub async fn get_server_by_id(db_pool: &PgPool, id: Uuid) -> Result<Server, Error> {
    query_as(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(db_pool)
    .await
}

#[tracing::instrument(
    name = "Getting many servers by owner_id",
    skip(id, db_pool),
    fields(
        owner_id = %id
    )
)]
pub async fn get_many_servers_by_owner_id(
    db_pool: &PgPool,
    id: Uuid,
) -> Result<Vec<Server>, Error> {
    query_as(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE owner_id = $1
        "#
    )
    .bind(id)
    .fetch_all(db_pool)
    .await
}

#[tracing::instrument(
    name = "Soft Deleting Server in Database",
    skip(server_id, deleted_at, db_pool),
    fields(
        server_id = %server_id,
        deleted_at = %deleted_at,
    )
)]
pub async fn soft_delete_server(
    db_pool: &PgPool,
    server_id: Uuid,
    deleted_at: DateTime<Utc>,
) -> Result<PgQueryResult, Error> {
    query(
        r#"
            UPDATE servers SET deleted_at = $1 WHERE id = $2;
        "#,
    )
    .bind(deleted_at)
    .bind(server_id)
    .execute(db_pool)
    .await
}

#[tracing::instrument(
    name = "Hard Deleting Server in Database",
    skip(server_id, db_pool),
    fields(
        server_id = %server_id,
    )
)]
pub async fn hard_delete_server(db_pool: &PgPool, server_id: Uuid) -> Result<PgQueryResult, Error> {
    query(
        r#"
            DELETE FROM servers WHERE id = $1;
        "#,
    )
    .bind(server_id)
    .execute(db_pool)
    .await
}

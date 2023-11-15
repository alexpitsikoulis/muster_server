use std::fmt::Display;

use crate::domain::server::{AsServer, Server};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub const SERVERS_TABLE_NAME: &str = "servers";

#[tracing::instrument(
    name = "Upserting server details to database",
    skip(server, db_pool),
    fields(
        server_data = %server,
    )
)]
pub async fn upsert_server<S>(db_pool: &PgPool, server: &S) -> Result<(), Error>
where
    S: AsServer + Display + std::fmt::Debug,
{
    sqlx::query!(
        r#"
        INSERT INTO servers (id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (id)
        DO
            UPDATE SET
                name = COALESCE($10, servers.name),
                owner_id = COALESCE($11, servers.owner_id),
                description = COALESCE(EXCLUDED.description, servers.description),
                photo = COALESCE(EXCLUDED.photo, servers.photo),
                cover_photo = COALESCE(EXCLUDED.cover_photo, servers.cover_photo),
                updated_at = now(),
                deleted_at = COALESCE(EXCLUDED.deleted_at, servers.deleted_at)
        WHERE
            (servers.name, servers.owner_id, servers.description, servers.photo, servers.cover_photo, servers.deleted_at) IS DISTINCT FROM
            (EXCLUDED.name, EXCLUDED.owner_id, EXCLUDED.description, EXCLUDED.photo, EXCLUDED.cover_photo, EXCLUDED.deleted_at);

        "#,
        server.id(),
        server.name(),
        server.owner_id(),
        server.description(),
        server.photo(),
        server.cover_photo(),
        server.created_at(),
        server.updated_at(),
        server.deleted_at(),
        match server.name() == String::new() {
            true => None,
            false => Some(server.name()),
        },
        match server.owner_id() == Uuid::nil() {
            true => None,
            false => Some(server.owner_id()),
        },
    )
    .execute(db_pool)
    .await
    .map(|_| {
        tracing::info!("UPSERT server {:?} successful", server);
    })
    .map_err(|e| {
        tracing::error!("UPSERT server {:?} failed: {:?}", server, e);
        e
    })
}

#[tracing::instrument(
    name = "Getting server by id",
    skip(id, db_pool),
    fields(
        server_id = %id
    )
)]
pub async fn get_server_by_id(db_pool: &PgPool, id: Uuid) -> Result<Server, Error> {
    sqlx::query!(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE id = $1
        "#, id
    )
    .fetch_one(db_pool)
    .await
    .map(|s| {
        tracing::info!("GET server by id {} successful", id);
        Server::new(
            s.id,
            s.name,
            s.owner_id,
            s.description,
            s.photo,
            s.cover_photo,
            s.created_at,
            s.updated_at,
            s.deleted_at,
        )
    })
    .map_err(|e| {
        tracing::error!("GET server by id {} failed: {:?}", id, e);
        e
    })
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
    sqlx::query!(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE owner_id = $1
        "#, id
    )
    .fetch_all(db_pool)
    .await
    .map(|s| {
        tracing::info!("GET many servers by owner_id {} successful", id);
        let mut servers: Vec<Server> = Vec::new();
        for record in s {
            servers.push(Server::new(
                record.id,
                record.name,
                record.owner_id,
                record.description,
                record.photo,
                record.cover_photo,
                record.created_at,
                record.updated_at,
                record.deleted_at,
            ))
        }
        servers
    })
    .map_err(|e| {
        tracing::error!("GET many servers by owner_id {} failed: {:?}", id, e);
        e
    })
}

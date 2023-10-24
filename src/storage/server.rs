use sqlx::{PgPool, Error};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::handlers::CreateServerRequestDataWithOwner;

pub const SERVERS_TABLE_NAME: &str = "servers";

pub struct Server {
    id: Uuid,
    name: String,
    owner_id: Uuid,
    description: Option<String>,
    photo: Option<String>,
    cover_photo: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Server {
    pub fn new(
        id: Uuid,
        name: String,
        owner_id: Uuid,
        description: Option<String>,
        photo: Option<String>,
        cover_photo: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Server {
            id,
            name,
            owner_id,
            description,
            photo,
            cover_photo,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

impl Into<Server> for CreateServerRequestDataWithOwner {
    fn into(self) -> Server {
        let now = Utc::now();
        Server::new(
            Uuid::new_v4(),
            self.data.name.clone(),
            self.owner_id,
            self.data.description.clone(),
            self.data.photo.clone(),
            self.data.cover_photo.clone(),
            now,
            now,
            None,
        )
    }
}

pub async fn upsert_server(db_pool: &PgPool, server: &Server) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
        INSERT INTO servers (id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, now(), $8)
        ON CONFLICT (id)
        DO
            UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                photo = EXCLUDED.photo,
                cover_photo = EXCLUDED.photo,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at
        WHERE
            (servers.name, servers.description, servers.photo, servers.cover_photo, servers.deleted_at) IS DISTINCT FROM
            (EXCLUDED.name, EXCLUDED.description, EXCLUDED.photo, EXCLUDED.cover_photo, EXCLUDED.deleted_at)

        "#,
        server.id,
        server.name,
        server.owner_id,
        server.description,
        server.photo,
        server.cover_photo,
        server.created_at,
        server.deleted_at,
    )
    .execute(db_pool)
    .await
}

pub async fn get_server_by_id(db_pool: &PgPool, id: Uuid) -> Result<Server, Error> {
    match sqlx::query!(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE id = $1
        "#, id
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(server) => Ok(Server::new(
            server.id,
            server.name,
            server.owner_id,
            server.description,
            server.photo,
            server.cover_photo,
            server.created_at,
            server.updated_at,
            server.deleted_at,
        )),
        Err(e) => Err(e),
    }
}

pub async fn get_many_servers_by_owner_id(db_pool: &PgPool, id: Uuid) -> Result<Vec<Server>, Error> {
    match sqlx::query!(
        r#"
        SELECT id, name, owner_id, description, photo, cover_photo, created_at, updated_at, deleted_at
        FROM servers
        WHERE owner_id = $1
        "#, id
    )
    .fetch_all(db_pool)
    .await
    {
        Ok(records) => {
            let mut servers: Vec<Server> = Vec::new();
            for record in records {
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
            Ok(servers)
        },
        Err(e) => Err(e),
    }
}
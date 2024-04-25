use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::storage::{get_server_by_id, hard_delete_server, soft_delete_server};

#[tracing::instrument(
    name = "Soft Deleting Server",
    skip(server_id, db_pool),
    fields(
        id = %server_id,
    )
)]
pub async fn soft_delete(server_id: Path<Uuid>, db_pool: Data<PgPool>) -> HttpResponse {
    let now = Utc::now();
    let id = server_id.into_inner();

    match get_server_by_id(&db_pool, id).await {
        Ok(server) => {
            if server.deleted_at().is_some() {
                let err = format!("server {} has already been soft deleted", id);
                tracing::error!(err);
                HttpResponse::BadRequest().body(err)
            } else {
                match soft_delete_server(&db_pool, id, now).await {
                    Ok(_) => {
                        tracing::info!("server {} successfully soft deleted", id);
                        HttpResponse::Ok().finish()
                    }
                    Err(e) => {
                        let err = format!("failed to soft delete server {}: {}", id, e);
                        tracing::error!(err);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let err = format!("server {} not found", id);
                tracing::error!(err);
                HttpResponse::NotFound().body(err)
            }
            e => {
                let err = format!("failed to soft delete server {}: {}", id, e);
                tracing::error!(err);
                HttpResponse::InternalServerError().body(err)
            }
        },
    }
}

#[tracing::instrument(
    name = "Hard Deleting Server",
    skip(server_id, db_pool),
    fields(
        id = %server_id,
    )
)]
pub async fn hard_delete(server_id: Path<Uuid>, db_pool: Data<PgPool>) -> HttpResponse {
    let id = server_id.into_inner();
    match hard_delete_server(&db_pool, id).await {
        Ok(_) => {
            tracing::info!("server {} successfully hard deleted", id);
            HttpResponse::Ok().finish()
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let err = format!("server {} not found", id);
                tracing::error!(err);
                HttpResponse::NotFound().body(err)
            }
            e => {
                let err = format!("failed to hard delete server {}: {}", id, e);
                tracing::error!(err);
                HttpResponse::InternalServerError().body(err)
            }
        },
    }
}

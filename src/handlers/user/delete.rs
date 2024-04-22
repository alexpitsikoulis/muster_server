use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::storage::{get_user_by_id, hard_delete_user, soft_delete_user};

#[tracing::instrument(
    name = "Soft Deleting User",
    skip(user_id, db_pool),
    fields(
        id = %user_id,
    )
)]
pub async fn soft_delete(user_id: Path<Uuid>, db_pool: Data<PgPool>) -> HttpResponse {
    let now = Utc::now();
    let id = user_id.into_inner();

    match get_user_by_id(&db_pool, id).await {
        Ok(user) => {
            if user.deleted_at().is_some() {
                let err = format!("user {} has already been soft deleted", id);
                tracing::error!(err);
                HttpResponse::BadRequest().body(err)
            } else {
                match soft_delete_user(&db_pool, id, now).await {
                    Ok(_) => {
                        tracing::info!("user {} successfully soft deleted", id);
                        HttpResponse::Ok().finish()
                    }
                    Err(e) => {
                        let err = format!("failed to soft delete user {}: {:?}", id, e);
                        tracing::error!(err);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let err = format!("user {} not found", id);
                tracing::error!(err);
                HttpResponse::NotFound().body(err)
            }
            e => {
                let err = format!("failed to soft delete user {}: {:?}", id, e);
                tracing::error!(err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[tracing::instrument(
    name = "Hard Deleting User",
    skip(user_id, db_pool),
    fields(
        user_id = %user_id,
    )
)]
pub async fn hard_delete(user_id: Path<Uuid>, db_pool: Data<PgPool>) -> HttpResponse {
    let id = user_id.into_inner();
    match hard_delete_user(&db_pool, id).await {
        Ok(_) => {
            tracing::info!("user {} successfully hard deleted", id);
            HttpResponse::Ok().finish()
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let err = format!("user {} not found", id);
                tracing::error!(err);
                HttpResponse::NotFound().body(err)
            }
            e => {
                let err = format!("failed to hard delete user {}: {:?}", id, e);
                tracing::error!(err);
                HttpResponse::InternalServerError().body(err)
            }
        },
    }
}

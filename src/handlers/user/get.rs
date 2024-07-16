use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::user::GetUserResponse, storage};

#[tracing::instrument(
    name = "Getting user by ID",
    skip(user_id, db_pool),
    fields(
        id = %user_id,
    ),
)]
pub async fn get_by_id(user_id: Path<Uuid>, db_pool: Data<PgPool>) -> HttpResponse {
    let id = user_id.into_inner();

    match storage::get_user_by_id(&db_pool, id).await {
        Ok(user) => {
            if user.deleted_at().is_some() {
                let err = format!("user {} has been soft deleted", id);
                tracing::error!(err);
                HttpResponse::BadRequest().body(err)
            } else {
                let response: GetUserResponse = user.into();
                let response_str = match serde_json::to_string(&response) {
                    Ok(response_str) => response_str,
                    Err(e) => {
                        let err = format!("failed to parse user struct {} to json: {:?}", id, e);
                        tracing::error!(err);
                        return HttpResponse::InternalServerError().finish();
                    }
                };
                HttpResponse::Ok().body(response_str)
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let err = format!("user {} not found", id);
                tracing::error!(err);
                HttpResponse::NotFound().body(err)
            }
            e => {
                let err = format!("failed to get user {}: {:?}", id, e);
                tracing::error!(err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

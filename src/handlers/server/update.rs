use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::server::Server, storage::upsert_server};

#[tracing::instrument(
    name = "Updating server details",
    skip(server_id, server_details, db_pool),
    fields(
        id = %server_id,
        name = %server_details.clone().name(),
        owner_id = %server_details.clone().owner_id(),
        description = %server_details.clone().description().unwrap_or_default(),
        photo = %server_details.clone().photo().unwrap_or_default(),
        cover_photo = %server_details.clone().cover_photo().unwrap_or_default(),
    )
)]
pub async fn update(
    server_id: Path<Uuid>,
    mut server_details: Json<Server>,
    db_pool: Data<PgPool>,
) -> HttpResponse {
    let id = server_id.into_inner();
    server_details.set_id(id);
    match upsert_server(db_pool.get_ref(), &server_details).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().body("Server not found"),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

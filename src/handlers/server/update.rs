use std::str::FromStr;

use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::server::ServerUpdate, storage::upsert_server};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdateServerRequestData {
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub cover_photo: Option<String>,
}

#[tracing::instrument(
    name = "Updating server details",
    skip(req, server_details, db_pool),
    fields(
        name = %server_details.clone().name.unwrap_or(String::new()),
        owner_id = %server_details.clone().owner_id.unwrap_or(String::new()),
        description = %server_details.clone().description.unwrap_or(String::new()),
        photo = %server_details.clone().photo.unwrap_or(String::new()),
        cover_photo = %server_details.clone().cover_photo.unwrap_or(String::new()),
    )
)]
pub async fn update(
    req: HttpRequest,
    server_details: Json<UpdateServerRequestData>,
    db_pool: Data<PgPool>,
) -> HttpResponse {
    match req.match_info().get("server_id") {
        Some(server_id) => {
            match Uuid::from_str(server_id) {
                Ok(server_id) => {
                    match ServerUpdate::from_request(&server_details, server_id) {
                        Ok(server_update) => {
                            match upsert_server(db_pool.get_ref(), &server_update).await {
                                Ok(()) => HttpResponse::Ok().finish(),
                                Err(e) => match e {
                                    sqlx::Error::RowNotFound => HttpResponse::NotFound().body("Server not found"),
                                    _ => HttpResponse::InternalServerError().finish(),
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to parse owner_id '{}' into UUID: {:?}", server_details.clone().owner_id.unwrap(), e);
                            HttpResponse::BadRequest().body(format!("Provided owner_id is invalid, please provide a valid UUID"))
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Invalud URL parameter '{}' for server_id provided: {:?}", server_id, e);
                    HttpResponse::BadRequest().body("URL parameter for server_id is incorrect, please provide a valid UUID")
                },
            }
        },
        None => HttpResponse::BadRequest().body("No URL parameter for server_id was provided. Request URL should be '/servers/update/{server_id}'"),
    }
}

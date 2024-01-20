use crate::{domain::server::Server, storage::upsert_server, utils::jwt::get_claims_from_token};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateServerRequestData {
    pub name: String,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub cover_photo: Option<String>,
}

#[tracing::instrument(
    name = "Creating new server",
    skip(server_details, db_pool),
    fields(
        server_name = %server_details.name.clone(),
        server_description = %server_details.description.clone().unwrap_or_default(),
        server_photo = %server_details.photo.clone().unwrap_or_default(),
        server_cover_photo = %server_details.cover_photo.clone().unwrap_or_default(),
    )
)]
pub async fn create(
    Json(server_details): Json<CreateServerRequestData>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    match req.headers().get("Authorization") {
        Some(header) => {
            let token = match header.to_str() {
                Ok(t) => t.to_string(),
                Err(e) => {
                    let error =
                        format!("Authorization header could not be cast to string: {:?}", e);
                    tracing::error!("500 - {}", error);
                    return HttpResponse::BadRequest().body(error);
                }
            };
            match get_claims_from_token(token) {
                Ok(claims) => {
                    let subject = claims.sub;
                    if server_details.name.len() > 50 {
                        let error = "Server name is too long, must be no more than 50 characters";
                        tracing::error!("400 - {}", error);
                        return HttpResponse::BadRequest().body(error);
                    }
                    let owner_id = match Uuid::parse_str(&subject) {
                        Ok(sub) => sub,
                        Err(e) => {
                            let error =
                                format!("Malformed JWT provided, sub is not a valid UUID: {:?}", e);
                            tracing::error!("400 - {}", error);
                            return HttpResponse::BadRequest().body(error);
                        }
                    };
                    let server = Server::from_create_request(server_details, owner_id);

                    match upsert_server(db_pool.get_ref(), &server).await {
                        Ok(_) => HttpResponse::Ok().body(server.id().to_string()),
                        Err(e) => {
                            tracing::error!("500 - Failed to execute query: {}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("403 - Failed to get claims from JWT: {:?}", e);
                    return HttpResponse::Forbidden().finish();
                }
            }
        }
        None => {
            tracing::error!("401 - No Authorization header provided");
            return HttpResponse::Unauthorized().finish();
        }
    }
}

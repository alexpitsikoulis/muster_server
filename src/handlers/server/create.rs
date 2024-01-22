use crate::{domain::server::Server, storage::upsert_server, utils::jwt::get_claims_from_token};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(
    name = "Creating new server",
    skip(server, db_pool),
    fields(
        server_name = %server.name().clone(),
        server_description = %server.description().clone().unwrap_or_default(),
        server_photo = %server.photo().clone().unwrap_or_default(),
        server_cover_photo = %server.cover_photo().clone().unwrap_or_default(),
    )
)]
pub async fn create(
    Json(mut server): Json<Server>,
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
                    if server.name().len() > 50 {
                        let error = "Server name is too long, must be no more than 50 characters";
                        tracing::error!("400 - {}", error);
                        return HttpResponse::BadRequest().body(error);
                    }
                    server.set_owner_id(match Uuid::parse_str(&subject) {
                        Ok(sub) => sub,
                        Err(e) => {
                            let error =
                                format!("Malformed JWT provided, sub is not a valid UUID: {:?}", e);
                            tracing::error!("400 - {}", error);
                            return HttpResponse::BadRequest().body(error);
                        }
                    });

                    match upsert_server(db_pool.get_ref(), &server).await {
                        Ok(_) => {
                            tracing::error!(
                                "Server {} successfull inserted to database",
                                server.id()
                            );
                            HttpResponse::Ok().body(server.id().to_string())
                        }
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

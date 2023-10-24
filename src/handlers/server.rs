use actix_web::{HttpResponse, web, HttpRequest};
use sqlx::PgPool;
use uuid::{Uuid, uuid};
use crate::storage::{upsert_server, Server};
use crate::utils::{get_claims_from_token, Claims};

#[derive(serde::Deserialize)]
pub struct CreateServerRequestData {
    pub name: String,
    pub description: Option<String>,
    pub photo: Option<String>,
    pub cover_photo: Option<String>,
}

pub struct CreateServerRequestDataWithOwner {
    pub data: web::Json<CreateServerRequestData>,
    pub owner_id: Uuid,
}

pub async fn create_server(body: web::Json<CreateServerRequestData>, db_pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let request_id = Uuid::new_v4();
    match req.headers().get("Authorization") {
        Some(header) => {
            let token =  match header.to_str() {
                Ok(t) => t.to_string(),
                Err(e) => {
                    tracing::error!("request_id {}: 500 - Authorization header could not be cast to string", request_id);
                    return HttpResponse::BadRequest().body("Authorization header could not be cast to string");
                }
            };
            match get_claims_from_token(token) {
                Ok(claims) => {
                    let subject = claims.sub;
                    if body.name.len() > 50 {
                        tracing::error!("request_id {}: 400 - Server name is too long, must be no more than 50 characters", request_id);
                        return HttpResponse::BadRequest().body("Server name is too long, must be no more than 50 characters");
                    }
                    let server: Server = CreateServerRequestDataWithOwner {
                        data: body,
                        owner_id: match Uuid::parse_str(&subject) {
                            Ok(sub) => sub,
                            Err(_) => {
                                tracing::error!("request_id {}: 400 -  Malformed JWT provided, sub is not a valid UUID", request_id);
                                return HttpResponse::BadRequest().body("Malformed JWT provided, sub is not a valid UUID")
                            },
                        },
                    }.into();
                
                    match upsert_server(db_pool.get_ref(), &server).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(e) => {
                            tracing::error!("request_id {}: 500 - Failed to execute query: {}", request_id, e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("request_id {}: 403 - Failed to get claims from JWT: {}", request_id, e);
                    return HttpResponse::Forbidden().finish()
                },
            }
        },
        None => {
            tracing::error!("request_id {}: 401 - No Authorization header provided", request_id);
            return HttpResponse::Unauthorized().finish()
        },
    }
}
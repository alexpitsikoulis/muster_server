use actix_web::{HttpResponse, web, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    storage::{upsert_server, Server},
    utils::jwt::get_claims_from_token,
};

#[derive(serde::Deserialize, Clone)]
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

#[tracing::instrument(
    name = "Creating new server",
    skip(body, db_pool),
    fields(
        server_name = %body.name,
    )
)]
pub async fn create_server(body: web::Json<CreateServerRequestData>, db_pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    match req.headers().get("Authorization") {
        Some(header) => {
            let token =  match header.to_str() {
                Ok(t) => t.to_string(),
                Err(e) => {
                    tracing::error!("500 - Authorization header could not be cast to string: {:?}", e);
                    return HttpResponse::BadRequest().body(format!("Authorization header could not be cast to string: {:?}", e));
                }
            };
            match get_claims_from_token(token) {
                Ok(claims) => {
                    let subject = claims.sub;
                    if body.name.len() > 50 {
                        tracing::error!("400 - Server name is too long, must be no more than 50 characters");
                        return HttpResponse::BadRequest().body("Server name is too long, must be no more than 50 characters");
                    }
                    let server: Server = CreateServerRequestDataWithOwner {
                        data: body,
                        owner_id: match Uuid::parse_str(&subject) {
                            Ok(sub) => sub,
                            Err(_) => {
                                tracing::error!("400 -  Malformed JWT provided, sub is not a valid UUID");
                                return HttpResponse::BadRequest().body("Malformed JWT provided, sub is not a valid UUID")
                            },
                        },
                    }.into();
                
                    match upsert_server(db_pool.get_ref(), &server).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(e) => {
                            tracing::error!("500 - Failed to execute query: {}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("403 - Failed to get claims from JWT: {}", e);
                    return HttpResponse::Forbidden().finish()
                },
            }
        },
        None => {
            tracing::error!("401 - No Authorization header provided");
            return HttpResponse::Unauthorized().finish()
        },
    }
}
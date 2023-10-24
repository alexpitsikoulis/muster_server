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
    match req.headers().get("Authorization") {
        Some(header) => {
            let token = header
                .to_str()
                .expect("Failed to cast Authorization header to string")
                .to_string();
            match get_claims_from_token(token) {
                Ok(claims) => {
                    let subject = claims.sub;
                    if body.name.len() > 50 {
                        return HttpResponse::BadRequest().body("Server name is too long, must be no more than 50 characters");
                    }
                    let server: Server = CreateServerRequestDataWithOwner {
                        data: body,
                        owner_id: match Uuid::parse_str(&subject) {
                            Ok(sub) => sub,
                            Err(_) => return HttpResponse::BadRequest().body("Malformed JWT provided, sub is not a valid UUID"),
                        },
                    }.into();
                
                    match upsert_server(db_pool.get_ref(), &server).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(e) => {
                            println!("Failed to execute query: {}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                Err(e) => return HttpResponse::Forbidden().finish(),
            }
        },
        None => return HttpResponse::Unauthorized().finish(),
    }
}
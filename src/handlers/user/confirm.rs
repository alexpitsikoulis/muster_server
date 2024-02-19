use crate::{
    storage::{confirm_user_email, delete, get_confirmation_token},
    utils::jwt::get_claims_from_token,
};
use actix_web::{web::Data, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

pub const CONFIRM_PATH: &str = "/confirm";

#[tracing::instrument(name = "Confirming user email", skip(req, db_pool), fields())]
pub async fn confirm(req: HttpRequest, db_pool: Data<PgPool>) -> HttpResponse {
    match req.match_info().get("confirmation_token") {
        Some(confirmation_token) => {
            match get_confirmation_token(&db_pool, confirmation_token).await {
                Ok(confirmation_token) => {
                    match get_claims_from_token(confirmation_token.expose().clone()) {
                        Ok(claims) => {
                            let now = Utc::now().timestamp() as usize;
                            if claims.iat > now {
                                tracing::error!(
                                    "Invalid token, issued_at time is later than current timestamp"
                                );
                                return HttpResponse::Unauthorized().body(
                                    "Invalid token, issued_at time is later than current timestamp",
                                );
                            }
                            if claims.sub != confirmation_token.user_id().to_string() {
                                tracing::error!("Confirmation token does not belong to this user");
                                return HttpResponse::Forbidden()
                                    .body("Confirmation token does not belong to this user");
                            };
                            if claims.exp <= now {
                                tracing::error!("Confirmation token is expired");
                                return HttpResponse::Unauthorized()
                                    .body("Confirmation token is expired");
                            };
                            if let Err(e) = delete(&db_pool, &confirmation_token).await {
                                tracing::error!("Failed to delete confirmation token: {:?}", e);
                                return HttpResponse::InternalServerError().finish();
                            }
                            match confirm_user_email(&db_pool, confirmation_token.user_id()).await {
                                Ok(_) => {
                                    tracing::error!(
                                        "Email successfully confirmed for user {}",
                                        confirmation_token.user_id()
                                    );
                                    return HttpResponse::Ok().finish();
                                }
                                Err(e) => match e {
                                    sqlx::Error::RowNotFound => {
                                        tracing::error!(
                                            "User not found for confirmation token user_id"
                                        );
                                        return HttpResponse::NotFound().body(format!(
                                            "User not found for confirmation token user_id {}",
                                            confirmation_token.user_id()
                                        ));
                                    }
                                    other => {
                                        tracing::error!("Failed to get user associated with confirmation token user_id {}: {:?}", confirmation_token.user_id(), other);
                                        return HttpResponse::InternalServerError().finish();
                                    }
                                },
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                "Failed to get claims from confirmation token: {:?}",
                                e
                            );
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        tracing::error!("Confirmation token not found");
                        HttpResponse::NotFound().finish()
                    }
                    other => {
                        tracing::error!("Failed to get confirmation token: {:?}", other);
                        HttpResponse::InternalServerError().finish()
                    }
                },
            }
        }
        None => {
            tracing::error!("No URL parameter for confirmation token was provided. Request URL should be '/confirm/{{confirmation_token}}'");
            HttpResponse::BadRequest().body("No URL parameter for confirmation token was provided. Request URL should be '/confirm/{confirmation_token}'")
        }
    }
}

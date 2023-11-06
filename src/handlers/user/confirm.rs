use actix_web::{
    HttpResponse,
    web::{Query, Data}
};
use chrono::Utc;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use crate::{
    domain::confirmation_token::GetConfirmationTokenData,
    storage::{ConfirmationToken, get_confirmation_token, confirm_user_email},
    utils::jwt::get_claims_from_token
};

#[tracing::instrument(
    name = "Confirming user email",
    skip(query, db_pool),
    fields(
        user_id = %query.user_id,
    )
)]
pub async fn confirm(query: Query<GetConfirmationTokenData>, db_pool: Data<PgPool>) -> HttpResponse {
    match ConfirmationToken::try_from(query) {
        Ok(confirmation_token) => {
            match get_confirmation_token(&db_pool, confirmation_token.confirmation_token(), confirmation_token.user_id())
                .await
                {
                    Ok(confirmation_token) => {
                        match get_claims_from_token(confirmation_token.confirmation_token().expose_secret().clone()) {
                            Ok(claims) => {
                                let now = Utc::now().timestamp() as usize;
                                if claims.iat > now {
                                    tracing::error!("Invalid token, issued_at time is later than current timestamp");
                                    return HttpResponse::Unauthorized().body("Invalid token, issued_at time is later than current timestamp")
                                }
                                if claims.sub != confirmation_token.user_id().to_string() {
                                    tracing::error!("Confirmation token does not belong to this user");
                                    return HttpResponse::Forbidden().body("Confirmation token does not belong to this user")
                                };
                                if claims.exp <= now {
                                    tracing::error!("Confirmation token is expired");
                                    return HttpResponse::Unauthorized().body("Confirmation token is expired")
                                };
                                match confirm_user_email(&db_pool, confirmation_token.user_id())
                                    .await
                                    {
                                        Ok(()) => return HttpResponse::Ok().finish(),
                                        Err(e) => match e {
                                            sqlx::Error::RowNotFound => {
                                                tracing::error!("User not found for confirmation token user_id");
                                                return HttpResponse::NotFound().body(format!("User not found for confirmation token user_id {}", confirmation_token.user_id()))
                                            },
                                            other => {
                                                tracing::error!("Failed to get user associated with confirmation token user_id {}: {:?}", confirmation_token.user_id(), other);
                                                return HttpResponse::InternalServerError().finish()
                                            }
                                        }
                                    }
                            },
                            Err(e) => {
                                tracing::error!("Failed to get claims from confirmation token: {:?}", e);
                                HttpResponse::InternalServerError().finish()
                            }
                        }
                    },
                    Err(e) => match e {
                        sqlx::Error::RowNotFound => {
                            tracing::error!("Confirmation token not found");
                            HttpResponse::NotFound().finish()
                        },
                        other => {
                            tracing::error!("Failed to get confirmation token: {:?}", other);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
        },
        Err(e) => {
            tracing::error!("Failed to parse user_id: {}", e);
            HttpResponse::BadRequest().body(e)
        },
    }
}

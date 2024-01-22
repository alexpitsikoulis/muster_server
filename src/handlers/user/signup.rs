use crate::{
    domain::{
        confirmation_token::ConfirmationToken,
        email,
        user::{Email, Handle, Password, User},
    },
    storage::{insert_confirmation_token, upsert_user},
    utils::jwt::generate_token,
};
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub const SIGNUP_PATH: &str = "/users/signup";

#[derive(Serialize, Deserialize)]
pub struct UserSignupFormData {
    #[serde(default)]
    pub id: Uuid,
    pub email: Email,
    pub handle: Handle,
    pub password: Password,
}

#[tracing::instrument(
    name = "Signing up new user",
    skip(form, db_pool, email_client),
    fields(
        id = %form.id,
        email = %form.email.as_ref(),
        handle = %form.handle.as_ref(),
    )
)]
pub async fn signup(
    form: Form<UserSignupFormData>,
    db_pool: Data<PgPool>,
    email_client: Data<email::Client>,
) -> HttpResponse {
    let user = User::from(form.into_inner());
    match upsert_user(db_pool.get_ref(), &user).await {
        Ok(_) => {
            tracing::info!("User {} successfully inserted to database", user.id());
            match generate_token(user.id()) {
                Ok(token) => {
                    let confirmation_token = ConfirmationToken::new(Secret::new(token), user.id());
                    match insert_confirmation_token(&db_pool, &confirmation_token).await {
                        Ok(_) => {
                            tracing::info!(
                                "Successfully inserted confirmation_token for user {}",
                                user.id()
                            );
                            match email_client
                                .send_confirmation_email(user.email(), &confirmation_token.inner())
                                .await
                            {
                                Ok(()) => {
                                    tracing::info!(
                                        "Confirmation email for user {} sent successfully",
                                        user.id()
                                    );
                                    HttpResponse::Ok().finish()
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to send confirmation email for user {}: {:?}",
                                        user.id(),
                                        e
                                    );
                                    HttpResponse::InternalServerError().finish()
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                "Failed to insert confirmation token for user {}: {:?}",
                                user.id(),
                                e
                            );
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to generate confirmation token for user {}: {:?}",
                        user.id(),
                        e
                    );
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to upsert user {:?} to database: {:?}", user, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

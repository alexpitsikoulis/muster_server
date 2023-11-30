use crate::{
    domain::{confirmation_token::ConfirmationToken, email, user::User},
    storage::{insert_confirmation_token, upsert_user},
    utils::jwt::generate_token,
};
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use secrecy::Secret;
use sqlx::PgPool;

pub const SIGNUP_PATH: &str = "/users/signup";

#[derive(serde::Deserialize, Clone)]
pub struct SignupFormData {
    pub email: String,
    pub handle: String,
    pub password: Secret<String>,
}

#[tracing::instrument(
    name = "Signing up new user",
    skip(form, db_pool, email_client),
    fields(
        user_email = %form.email,
        user_handle = %form.handle,
    )
)]
pub async fn signup(
    form: Form<SignupFormData>,
    db_pool: Data<PgPool>,
    email_client: Data<email::Client>,
) -> HttpResponse {
    let user = match User::try_from(form) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to validate new user: {:?}", e);
            return e.handle_http();
        }
    };

    match upsert_user(db_pool.get_ref(), &user).await {
        Ok(()) => {
            tracing::info!("User {:?} successfully inserted to database", user);
            match generate_token(user.id()) {
                Ok(token) => {
                    let confirmation_token = ConfirmationToken::new(Secret::new(token), user.id());
                    match insert_confirmation_token(&db_pool, &confirmation_token).await {
                        Ok(()) => {
                            match email_client
                                .send_confirmation_email(
                                    user.email(),
                                    confirmation_token.confirmation_token(),
                                )
                                .await
                            {
                                Ok(()) => HttpResponse::Ok().finish(),
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
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

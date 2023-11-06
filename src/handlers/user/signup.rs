use actix_web::{
    HttpResponse,
    web::{Data, Form}
};
use sqlx::PgPool;
use secrecy::Secret;
use crate::{
    domain::{
        user::User,
        mailer::Mailer,
    },
    storage::upsert_user,
};

#[derive(serde::Deserialize, Clone)]
pub struct SignupFormData {
    pub email: String,
    pub handle: String,
    pub password: Secret<String>,
}

#[tracing::instrument(
    name = "Signing up new user",
    skip(form, db_pool),
    fields(
        user_email = %form.email,
        user_handle = %form.handle,
    )
)]
pub async fn signup(form: Form<SignupFormData>, db_pool: Data<PgPool>) -> HttpResponse {
    let user = match User::try_from(form) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to validate new user: {:?}", e);
            return e.handle_http()
        },
    };

    match upsert_user(db_pool.get_ref(), &user).await {
        Ok(()) => {
            tracing::info!("User {:?} successfully inserted to database", user);
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
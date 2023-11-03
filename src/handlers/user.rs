use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use secrecy::Secret;
use crate::{
    domain::user::{NewUser, UserPassword},
    storage::{upsert_user, get_user_by_email, User},
    utils::jwt::generate_token,
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
pub async fn signup(form: web::Form<SignupFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let new_user = match NewUser::parse(form) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to validate new user: {:?}", e);
            return e.handle_http()
        },
    };

    let user: User = new_user.into();

    match upsert_user(db_pool.get_ref(), &user).await {
        Ok(()) => {
            tracing::info!("User {:?} successfully inserted to database", user);
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    email: String,
    password: Secret<String>,
}

#[tracing::instrument(
    name = "Logging in user",
    skip(form, db_pool),
    fields(
        user_email = %form.email,
    )
)]
pub async fn login(form: web::Form<LoginFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match get_user_by_email(db_pool.get_ref(), form.email.clone()).await
    {
        Ok(mut user) => {
            if user.failed_attempts >= 10 {
                return HttpResponse::Forbidden().body("Account is locked due to too many failed login attempts")
            };
            let login_successful = UserPassword::compare(form.password.clone(), user.password.to_string());
            if login_successful {
                user.failed_attempts = 0;
            } else {
                user.failed_attempts += 1;
            }
            if let Err(e) = upsert_user(db_pool.get_ref(), &user).await {
                tracing::error!("INSERT into users table failed: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }

            match generate_token(user.id) {
                Ok(token) => {
                    HttpResponse::Ok()
                        .append_header(("X-Login-Successful", login_successful.to_string()))
                        .append_header(("Authorization", token))
                        .finish()
                },
                Err(e) => {
                    tracing::error!("Failed to generate JWT: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }

        },
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => HttpResponse::Ok().append_header(("X-Login-Successful", "false")).finish(),
                _ => {
                    tracing::error!("Failed to execute GET from users table query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
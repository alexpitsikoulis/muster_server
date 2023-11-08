use actix_web::{
    HttpResponse,
    web::{Form, Data}
};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    domain::user::{Password, LoginData, Login},
    storage::{upsert_user, get_user_by_email, get_user_by_handle},
    utils::jwt::generate_token
};

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub login: String,
    pub password: Secret<String>,
}

#[tracing::instrument(
    name = "Logging in user",
    skip(form, db_pool),
    fields(
        user_login_option = %form.login,
    )
)]
pub async fn login(form: Form<LoginForm>, db_pool: Data<PgPool>) -> HttpResponse {
    let login_data = match LoginData::try_from(form) {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to parse login details");
            return HttpResponse::BadRequest().body(e)
        },
    };
    let query_result = match login_data.login {
        Login::Email(e) => get_user_by_email(db_pool.get_ref(), e).await,
        Login::Handle(h) => get_user_by_handle(db_pool.get_ref(), h).await,
    };
    match query_result {
        Ok(mut user) => {
            let mut changed = false;
            if !user.email_confirmed() {
                return HttpResponse::Unauthorized().body("Account email has not been confirmed");
            }
            if user.failed_attempts() >= 10 {
                return HttpResponse::Forbidden().body("Account is locked due to too many failed login attempts")
            };
            let login_successful = Password::compare(login_data.password.clone(), user.password().to_string());
            if login_successful {
                if user.failed_attempts() > 0 {
                    changed = true;
                    user.reset_failed_attempts();
                }
            } else {
                changed = true;
                user.increment_failed_attempts();
            }

            if changed {
                if let Err(e) = upsert_user(db_pool.get_ref(), &user).await {
                    tracing::error!("INSERT into users table failed: {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }

            match generate_token(user.id()) {
                Ok(token) => {
                    HttpResponse::Ok()
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
                sqlx::Error::RowNotFound => HttpResponse::Ok().finish(),
                _ => {
                    tracing::error!("Failed to execute GET from users table query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::{validate_and_hash_password, compare_password_hash, PasswordValidationError, generate_token};
use crate::storage::{upsert_user, get_user_by_email, User};

#[derive(serde::Deserialize, Clone)]
pub struct SignupFormData {
    pub email: String,
    pub handle: String,
    pub password: String,
}

#[tracing::instrument(
    name = "Signing up new user",
    skip(form, db_pool),
    fields(
        request_id = %Uuid::new_v4(),
        user_email = %form.email,
        user_handle = %form.handle,
    )
)]
pub async fn signup(form: web::Form<SignupFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match validate_and_hash_password(form.password.clone()) {
        Ok(password_hash) => {
            let mut user_data: User = form.into();
            user_data.password = password_hash;
            match upsert_user(db_pool.get_ref(), &user_data).await
            {
                Ok(_) => HttpResponse::Ok().finish() ,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        },
        Err(e) => {
            match e {
                PasswordValidationError::PwdTooShort => HttpResponse::BadRequest().body("Password too short, must be at least 8 characters long"),
                PasswordValidationError::PwdTooLong => HttpResponse::BadRequest().body("Password too long, must be no longer than 64 characters"),
                PasswordValidationError::PwdMissingChar => HttpResponse::BadRequest().body("Password must contain at least one special character (\" # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \\ ] ^ _ ` { | } ~ )"),
                PasswordValidationError::PwdMissingNumber => HttpResponse::BadRequest().body("Password must contain at least one number"),
                PasswordValidationError::PwdMissingUppercase => HttpResponse::BadRequest().body("Password must contain at least one uppercase letter"),
                PasswordValidationError::PwdMissingLowercase => HttpResponse::BadRequest().body("Password must contain at least one lowercase letter"),
                PasswordValidationError::ArgonErr(e) => {
                    tracing::error!("Argon2 failed to validate password: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    email: String,
    password: String,
}

#[tracing::instrument(
    name = "Logging in user",
    skip(form, db_pool),
    fields(
        request_id = %Uuid::new_v4(),
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
            let login_successful = compare_password_hash(form.password.clone(), user.password.clone());
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
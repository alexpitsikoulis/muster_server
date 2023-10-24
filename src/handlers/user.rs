use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::{validate_and_hash_password, compare_password_hash, PasswordValidationError, generate_token};
use crate::storage::{upsert_user, get_user_by_email, User};

#[derive(serde::Deserialize)]
pub struct SignupFormData {
    pub email: String,
    pub name: String,
    pub password: String,
}

pub async fn signup(form: web::Form<SignupFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    match validate_and_hash_password(form.password.clone()) {
        Ok(password_hash) => {
            let mut user_data: User = form.into();
            user_data.password = password_hash;
            match upsert_user(db_pool.get_ref(), &user_data).await
            {
                Ok(_) => {
                    tracing::info!("request_id {}: INSERT into users ('{}', '{}') successful", request_id, user_data.email, user_data.name);
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    tracing::error!("request_id {}: Failed to execute query: {}", request_id, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(e) => {
            match e {
                PasswordValidationError::PwdTooShort => HttpResponse::BadRequest().body("Password too short, must be at least 8 characters long"),
                PasswordValidationError::PwdTooLong => HttpResponse::BadRequest().body("Password too long, must be no longer than 64 characters"),
                PasswordValidationError::PwdMissingChar => HttpResponse::BadRequest().body("Password must contain at least one special character (\" # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \\ ] ^ _ ` { | } ~ )"),
                PasswordValidationError::PwdMissingNumber => HttpResponse::BadRequest().body("Password must contain at least one number"),
                PasswordValidationError::PwdMissingUpperCase => HttpResponse::BadRequest().body("Password must contain at least one uppercase letter"),
                PasswordValidationError::PwdMissingLowercase => HttpResponse::BadRequest().body("Password must contain at least one lowercase letter"),
                PasswordValidationError::ArgonErr(e) => {
                    tracing::error!("request_id {}: Argon2 failed to validate password: {:?}", request_id, e);
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

pub async fn login(form: web::Form<LoginFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
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
                tracing::error!("request_id {}: INSERT into users table failed: {:?}", request_id, e);
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
                    tracing::error!("request_id {}: Failed to generate JWT: {}", request_id, e);
                    HttpResponse::InternalServerError().finish()
                }
            }

        },
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => HttpResponse::Ok().append_header(("X-Login-Successful", "false")).finish(),
                _ => {
                    tracing::error!("request_id {}: Failed to execute GET from users table query: {:?}", request_id, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
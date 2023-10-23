use std::convert::Into;

use actix_web::{HttpResponse, web, HttpResponseBuilder};
use sqlx::PgPool;
use crate::utils::{validate_and_hash_password, compare_password_hash, PasswordValidationError};
use crate::storage::{upsert_user, get_user_by_email, User};

#[derive(serde::Deserialize)]
pub struct SignupFormData {
    pub email: String,
    pub name: String,
    pub password: String,
}

pub async fn signup(form: web::Form<SignupFormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match validate_and_hash_password(form.password.clone()) {
        Ok(password_hash) => {
            let mut user_data: User = form.into();
            user_data.password = password_hash;
            match upsert_user(db_pool.get_ref(), &user_data).await
            {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => {
                    println!("Failed to execute query: {}", e);
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
                PasswordValidationError::ArgonErr(_) => HttpResponse::InternalServerError().finish(),
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
    match get_user_by_email(db_pool.get_ref(), form.email.clone()).await
    {
        Ok(mut user) => {
            let login_successful = compare_password_hash(form.password.clone(), user.password.clone());
            if login_successful {
                user.failed_attempts = 0;
                user.is_locked = false;
            } else {
                user.failed_attempts += 1;
                if user.failed_attempts >= 10 {
                    user.is_locked = true;
                }
            }
            if let Err(e) = upsert_user(db_pool.get_ref(), &user).await {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().append_header(
                ("X-Login-Successful", 
                    match compare_password_hash(form.password.clone(), user.password.clone()) {
                        true => "true",
                        false => "false"
                    }
                )
            ).finish()
        },
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => HttpResponse::Ok().append_header(("X-Login-Successful", "false")).finish(),
                _ => {
                    println!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
use actix_web::web;
use chrono::{Utc, DateTime};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::handlers::SignupFormData;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_locked: bool,
    pub failed_attempts: i16,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        name: String,
        password: String,
        profile_photo: Option<String>,
        bio: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
        is_locked: bool,
        failed_attempts: i16,
    ) -> Self {
        User {
            id,
            email,
            name,
            password,
            profile_photo,
            bio,
            created_at,
            updated_at,
            deleted_at,
            is_locked,
            failed_attempts,
        }
    }
}

impl Into<User> for web::Form<SignupFormData> {
    fn into(self) -> User {
        let now = Utc::now();
        User::new(
            Uuid::new_v4(),
            self.email.clone(),
            self.name.clone(),
            self.password.clone(),
            None,
            None,
            now,
            now,
            None,
            false,
            0,
        )
    }
}

pub async fn upsert_user(db_pool: &PgPool, user: &User) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, is_locked, failed_attempts)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (id)
        DO
            UPDATE SET
                email = EXCLUDED.email,
                name = EXCLUDED.name,
                password = EXCLUDED.password,
                profile_photo = EXCLUDED.profile_photo,
                bio = EXCLUDED.bio,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at,
                is_locked = EXCLUDED.is_locked,
                failed_attempts = EXCLUDED.failed_attempts
            WHERE
                (users.email, users.name, users.password, users.profile_photo, users.bio, users.deleted_at, users.is_locked, users.failed_attempts) IS DISTINCT FROM
                (EXCLUDED.email, EXCLUDED.name, EXCLUDED.password, EXCLUDED.profile_photo, EXCLUDED.bio, EXCLUDED.deleted_at, EXCLUDED.is_locked, EXCLUDED.failed_attempts)
        "#,
        user.id,
        user.email,
        user.name,
        user.password,
        user.profile_photo,
        user.bio,
        user.created_at,
        user.updated_at,
        user.deleted_at,
        user.is_locked,
        user.failed_attempts,
    )
    .execute(db_pool)
    .await
}

pub async fn get_user_by_id(db_pool: &PgPool, id: Uuid) -> Result<User, Error> {
    match sqlx::query!(
        r#"
        SELECT id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, is_locked, failed_attempts
        FROM users
        WHERE id = $1
        "#, id
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(user) => Ok(User::new(
            user.id,
            user.email,
            user.name,
            user.password,
            user.profile_photo,
            user.bio,
            user.created_at,
            user.updated_at,
            user.deleted_at,
            user.is_locked,
            user.failed_attempts,
        )),
        Err(e) => Err(e),
    }
}

pub async fn get_user_by_email(db_pool: &PgPool, email: String) -> Result<User, Error> {
    match sqlx::query!(
        r#"
        SELECT id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, is_locked, failed_attempts
        FROM users
        WHERE email = $1
        "#, email
    )
        .fetch_one(db_pool)
        .await
        {
            Ok(user) => Ok(User::new(
                user.id,
                user.email,
                user.name,
                user.password,
                user.profile_photo,
                user.bio,
                user.created_at,
                user.updated_at,
                user.deleted_at,
                user.is_locked,
                user.failed_attempts,
            )),
            Err(e) => Err(e)
        }
}
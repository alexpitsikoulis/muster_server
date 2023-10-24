use actix_web::web;
use chrono::{Utc, DateTime};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::handlers::SignupFormData;

pub const USERS_TABLE_NAME: &str = "users";

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        name: String,
        password: String,
        profile_photo: Option<String>,
        bio: Option<String>,
        failed_attempts: i16,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        User {
            id,
            email,
            name,
            password,
            profile_photo,
            bio,
            failed_attempts,
            created_at,
            updated_at,
            deleted_at,
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
            0,
            now,
            now,
            None,
        )
    }
}

pub async fn upsert_user(db_pool: &PgPool, user: &User) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, failed_attempts)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
                failed_attempts = EXCLUDED.failed_attempts
            WHERE
                (users.email, users.name, users.password, users.profile_photo, users.bio, users.deleted_at, users.failed_attempts) IS DISTINCT FROM
                (EXCLUDED.email, EXCLUDED.name, EXCLUDED.password, EXCLUDED.profile_photo, EXCLUDED.bio, EXCLUDED.deleted_at, EXCLUDED.failed_attempts)
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
        user.failed_attempts,
    )
    .execute(db_pool)
    .await
}

pub async fn get_user_by_id(db_pool: &PgPool, id: Uuid) -> Result<User, Error> {
    match sqlx::query!(
        r#"
        SELECT id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, failed_attempts
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
            user.failed_attempts,
            user.created_at,
            user.updated_at,
            user.deleted_at,
        )),
        Err(e) => Err(e),
    }
}

pub async fn get_user_by_email(db_pool: &PgPool, email: String) -> Result<User, Error> {
    match sqlx::query!(
        r#"
        SELECT id, email, name, password, profile_photo, bio, created_at, updated_at, deleted_at, failed_attempts
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
                user.failed_attempts,
                user.created_at,
                user.updated_at,
                user.deleted_at,
            )),
            Err(e) => Err(e)
        }
}
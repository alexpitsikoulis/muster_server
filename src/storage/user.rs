use chrono::{Utc, DateTime};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::user::NewUser;

pub const USERS_TABLE_NAME: &str = "users";

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub handle: String,
    pub name: Option<String>,
    pub password: String,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
    pub failed_attempts: i16,
    pub email_confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        handle: String,
        name: Option<String>,
        password: String,
        profile_photo: Option<String>,
        bio: Option<String>,
        failed_attempts: i16,
        email_confirmed: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        User {
            id,
            email,
            handle,
            name,
            password,
            profile_photo,
            bio,
            failed_attempts,
            email_confirmed,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        let now = Utc::now();
        User::new(
            Uuid::new_v4(),
            user.email.as_ref().to_string(),
            user.handle.as_ref().to_string(),
            None,
            user.password.as_ref().to_string(),
            None,
            None,
            0,
            false,
            now,
            now,
            None,
        )
    }
}

#[tracing::instrument(
    name = "Upserting user details to database",
    skip(user, db_pool),
)]
pub async fn upsert_user(db_pool: &PgPool, user: &User) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT (id)
        DO
            UPDATE SET
                email = EXCLUDED.email,
                handle = EXCLUDED.handle,
                name = EXCLUDED.name,
                password = EXCLUDED.password,
                profile_photo = EXCLUDED.profile_photo,
                bio = EXCLUDED.bio,
                email_confirmed = EXCLUDED.email_confirmed,
                updated_at = now(),
                deleted_at = EXCLUDED.deleted_at,
                failed_attempts = EXCLUDED.failed_attempts
            WHERE
                (users.email, users.name, users.password, users.profile_photo, users.bio, users.email_confirmed, users.deleted_at, users.failed_attempts) IS DISTINCT FROM
                (EXCLUDED.email, EXCLUDED.name, EXCLUDED.password, EXCLUDED.profile_photo, EXCLUDED.bio, EXCLUDED.email_confirmed, EXCLUDED.deleted_at, EXCLUDED.failed_attempts)
        "#,
        user.id,
        user.email,
        user.handle,
        user.name,
        user.password,
        user.profile_photo,
        user.bio,
        user.email_confirmed,
        user.created_at,
        user.updated_at,
        user.deleted_at,
        user.failed_attempts,
    )
    .execute(db_pool)
    .await
    .map(|_| {
        tracing::info!("UPSERT user {:?} query successful", user);
    })
    .map_err(|e| {
        tracing::error!("UPSERT user {:?} failed: {:?}", user, e);
        e
    })
}

#[tracing::instrument(
    name = "Getting user by id",
    skip(id, db_pool),
    fields(
        user_id = %id
    )
)]
pub async fn get_user_by_id(db_pool: &PgPool, id: Uuid) -> Result<User, Error> {
    sqlx::query!(
        r#"
        SELECT id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts
        FROM users
        WHERE id = $1
        "#, id
    )
    .fetch_one(db_pool)
    .await
    .map(|u| {
        tracing::info!("GET user by id {} successful", id);
        User::new(
            u.id,
            u.email,
            u.handle,
            u.name,
            u.password,
            u.profile_photo,
            u.bio,
            u.failed_attempts,
            u.email_confirmed,
            u.created_at,
            u.updated_at,
            u.deleted_at,
        )
    })
    .map_err(|e| {
        tracing::error!("GET user by id {} failed: {:?}", id, e); 
        e
    })
}

#[tracing::instrument(
    name = "Getting user by id",
    skip(email, db_pool),
    fields(
        user_email = %email
    )
)]
pub async fn get_user_by_email(db_pool: &PgPool, email: String) -> Result<User, Error> {
    sqlx::query!(
        r#"
        SELECT id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts
        FROM users
        WHERE email = $1
        "#, email
    )
        .fetch_one(db_pool)
        .await
        .map(|u| {
            tracing::info!("GET user by email {} successful", email);
            User::new(
                u.id,
                u.email,
                u.handle,
                u.name,
                u.password,
                u.profile_photo,
                u.bio,
                u.failed_attempts,
                u.email_confirmed,
                u.created_at,
                u.updated_at,
                u.deleted_at,
            )
        })
        .map_err(|e| {
            tracing::error!("GET user by email {} failed: {:?}", email, e);
            e
        })
}
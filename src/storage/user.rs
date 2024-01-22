use sqlx::{postgres::PgQueryResult, query, query_as, Error, PgPool};
use uuid::Uuid;

use crate::domain::user::User;

pub const USERS_TABLE_NAME: &str = "users";

#[tracing::instrument(name = "Upserting user details to database", skip(user, db_pool))]
pub async fn upsert_user(db_pool: &PgPool, user: &User) -> Result<PgQueryResult, Error> {
    query(
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
        "#
    )
        .bind(user.id())
        .bind(user.email().as_ref().to_string())
        .bind(user.handle().as_ref().to_string())
        .bind(user.name())
        .bind(user.password().as_ref())
        .bind(user.profile_photo())
        .bind(user.bio())
        .bind(user.email_confirmed())
        .bind(user.created_at())
        .bind(user.updated_at())
        .bind(user.deleted_at())
        .bind(user.failed_attempts())
    .execute(db_pool)
    .await
}

pub async fn patch_user<'a>(db_pool: &PgPool, q: String) -> Result<PgQueryResult, Error> {
    query(&q).execute(db_pool).await
}

#[tracing::instrument(
    name = "Setting user email_confirmed to true in database",
    skip(id, db_pool),
    fields(user_id = %id)
)]
pub async fn confirm_user_email(db_pool: &PgPool, id: Uuid) -> Result<PgQueryResult, Error> {
    query(
        r#"
        UPDATE users
        SET
            email_confirmed = true
        WHERE
            id = $1
        "#,
    )
    .bind(id)
    .execute(db_pool)
    .await
}

#[tracing::instrument(
    name = "Getting user by id",
    skip(id, db_pool),
    fields(
        user_id = %id
    )
)]
pub async fn get_user_by_id(db_pool: &PgPool, id: Uuid) -> Result<User, Error> {
    query_as(
        r#"
            SELECT id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts
            FROM users
            WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(db_pool)
    .await
}

#[tracing::instrument(
    name = "Getting user by email",
    skip(email, db_pool),
    fields(
        user_email = %email
    )
)]
pub async fn get_user_by_email(db_pool: &PgPool, email: &str) -> Result<User, Error> {
    query_as(
        r#"
            SELECT id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts
            FROM users
            WHERE email = $1
        "#
    )
    .bind(email)
        .fetch_one(db_pool)
        .await
}

#[tracing::instrument(
    name = "Getting user by handle",
    skip(handle, db_pool),
    fields(
        user_handle = %handle
    )
)]
pub async fn get_user_by_handle(db_pool: &PgPool, handle: &str) -> Result<User, Error> {
    query_as(
        r#"
            SELECT id, email, handle, name, password, profile_photo, bio, email_confirmed, created_at, updated_at, deleted_at, failed_attempts
            FROM users
            WHERE handle = $1
        "#)
        .bind(handle)
        .fetch_one(db_pool)
        .await
}

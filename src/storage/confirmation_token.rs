use crate::domain::confirmation_token::ConfirmationToken;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Error, PgPool};

#[tracing::instrument(
    name = "Inserting confirmation_token to database",
    skip(confirmation_token, db_pool),
    fields(
        user_id = %confirmation_token.user_id(),
    )
)]
pub async fn insert_confirmation_token(
    db_pool: &PgPool,
    confirmation_token: &ConfirmationToken,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO confirmation_tokens (confirmation_token, user_id)
        VALUES ($1, $2);
        "#,
        confirmation_token.expose(),
        confirmation_token.user_id(),
    )
    .execute(db_pool)
    .await
    .map(|_| {
        tracing::info!(
            "INSERT confirmation_token for user {} successful",
            confirmation_token.user_id()
        );
    })
    .map_err(|e| {
        tracing::info!(
            "INSERT confirmation_token for user {} failed: {:?}",
            confirmation_token.user_id(),
            e
        );
        e
    })
}

#[tracing::instrument(
    name = "Getting confirmation_token from database",
    skip(confirmation_token, db_pool),
    fields()
)]
pub async fn get_confirmation_token(
    db_pool: &PgPool,
    confirmation_token: &str,
) -> Result<ConfirmationToken, Error> {
    sqlx::query!(
        r#"
        SELECT confirmation_token, user_id
        FROM confirmation_tokens
        WHERE
            confirmation_token = $1;
        "#,
        confirmation_token,
    )
    .fetch_one(db_pool)
    .await
    .map(|t| {
        tracing::info!("GET confirmation_token successful");
        ConfirmationToken::new(Secret::new(t.confirmation_token), t.user_id)
    })
    .map_err(|e| {
        tracing::error!("GET confirmation_token failed: {:?}", e);
        e
    })
}

#[tracing::instrument(name = "Deleting confirmation token", skip(token, db_pool))]
pub async fn delete(db_pool: &PgPool, token: &ConfirmationToken) -> Result<(), Error> {
    sqlx::query!(
        r#"
        DELETE FROM confirmation_tokens
        WHERE
            confirmation_token = $1 AND
            user_id = $2;
        "#,
        token.confirmation_token().expose_secret(),
        token.user_id(),
    )
    .execute(db_pool)
    .await
    .map(|_| {
        tracing::info!(
            "DELETE confirmation token {} for user {} successful",
            token.confirmation_token().expose_secret(),
            token.user_id()
        );
    })
    .map_err(|e| {
        tracing::error!(
            "DELETE confirmation token for user {} failed",
            token.user_id()
        );
        e
    })
}

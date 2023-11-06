use secrecy::{Secret, ExposeSecret};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct ConfirmationToken {
    confirmation_token: Secret<String>,
    user_id: Uuid,
}

impl ConfirmationToken {
    pub fn new(confirmation_token: Secret<String>, user_id: Uuid) -> Self {
        ConfirmationToken { confirmation_token, user_id }
    }

    pub fn confirmation_token(&self) -> Secret<String> {
        self.confirmation_token.clone()
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

#[tracing::instrument(
    name = "Inserting confirmation_token to database",
    skip(confirmation_token, db_pool),
    fields(
        user_id = %confirmation_token.user_id,
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
        confirmation_token.confirmation_token.expose_secret(),
        confirmation_token.user_id,
    )
        .execute(db_pool)
        .await
        .map(|_| {
            tracing::info!("INSERT confirmation_token for user {} successful", confirmation_token.user_id);
        })
        .map_err(|e| {
            tracing::info!("INSERT confirmation_token for user {} failed: {:?}", confirmation_token.user_id, e);
            e
        })
}

#[tracing::instrument(
    name = "Getting confirmation_token from database",
    skip(user_id, confirmation_token, db_pool),
    fields(
        user_id = %user_id,
    )
)]
pub async fn get_confirmation_token(
    db_pool: &PgPool,
    confirmation_token: Secret<String>,
    user_id: Uuid
) -> Result<ConfirmationToken, Error> {
    sqlx::query!(
        r#"
        SELECT confirmation_token, user_id
        FROM confirmation_tokens
        WHERE
            confirmation_token = $1 AND
            user_id = $2;
        "#,
        confirmation_token.expose_secret(),
        user_id,
    )
        .fetch_one(db_pool)
        .await
        .map(|t| {
            tracing::info!("GET confirmation_token for user {} successful", user_id);
            ConfirmationToken::new(Secret::new(t.confirmation_token), t.user_id)
        })
        .map_err(|e| {
            tracing::error!("GET confirmation_token for user {} failed: {:?}", user_id, e);
            e
        })
}
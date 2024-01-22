use secrecy::Secret;
use sqlx::{postgres::PgTypeInfo, Decode, Postgres, Type};

use crate::domain::confirmation_token::ConfirmationTokenInner;

impl<'a> Decode<'a, Postgres> for ConfirmationTokenInner {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let token = String::decode(value)?;
        Ok(ConfirmationTokenInner::new(Secret::new(token)))
    }
}

impl Type<Postgres> for ConfirmationTokenInner {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}

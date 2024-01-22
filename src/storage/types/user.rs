use sqlx::{postgres::PgTypeInfo, Database, Decode, Postgres, Type};

use crate::domain::user::{Email, Handle, Password};

impl<'r> Decode<'r, Postgres> for Email {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let email = String::decode(value)?;
        Self::try_from(email)
            .map_err(|e| sqlx::error::BoxDynError::from(format!("failed to decode email: {:?}", e)))
    }
}

impl Type<Postgres> for Email {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }
}

impl<'r> Decode<'r, Postgres> for Handle {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let handle = String::decode(value)?;
        Self::try_from(handle).map_err(|e| {
            sqlx::error::BoxDynError::from(format!("failed to decode handle: {:?}", e))
        })
    }
}

impl Type<Postgres> for Handle {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }
}

impl<'r> Decode<'r, Postgres> for Password {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self::from_raw(String::decode(value)?))
    }
}

impl Type<Postgres> for Password {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}

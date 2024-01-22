use secrecy::Secret;
use sqlx::{postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};
use uuid::Uuid;

use crate::{
    domain::user::{Email, Handle, Password},
    handlers::user::PatchUserRequestBody,
};

impl<'r> Decode<'r, Postgres> for Email {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let email = String::decode(value)?;
        Self::try_from(email)
            .map_err(|e| sqlx::error::BoxDynError::from(format!("failed to decode email: {:?}", e)))
    }
}

impl<'q> Encode<'q, Postgres> for Email {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.as_ref().encode_by_ref(buf)
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

impl<'q> Encode<'q, Postgres> for Handle {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.as_ref().encode_by_ref(buf)
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
        Ok(Self::from_raw(Secret::new(String::decode(value)?)))
    }
}

impl<'q> Encode<'q, Postgres> for Password {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.as_ref().encode_by_ref(buf)
    }
}

impl Type<Postgres> for Password {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}

impl PatchUserRequestBody {
    pub fn build_query(&self, id: Uuid) -> Option<String> {
        let mut any_present = false;
        let mut q = String::from(
            r#"
            UPDATE users
            SET
            "#,
        );

        if let Some(email) = &self.email {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("email = '{}',", email));
        }
        if let Some(handle) = &self.handle {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("handle = '{}',", handle));
        }
        if let Some(password) = &self.password {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("password = '{}',", password.as_ref()));
        }
        if let Some(name) = &self.name {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("name = '{}',", name));
        }
        if let Some(profile_photo) = &self.profile_photo {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("profile_photo = '{}',", profile_photo));
        }
        if let Some(bio) = &self.bio {
            if !any_present {
                any_present = true
            };
            q.push_str(&format!("bio = '{}',", bio));
        }

        if any_present {
            Some(format!(
                r#"
            {}
            WHERE id = '{}';
            "#,
                q.trim_end_matches(','),
                id,
            ))
        } else {
            None
        }
    }
}

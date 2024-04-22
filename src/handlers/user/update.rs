use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::user::{
        deserialize_handle_option, deserialize_password_option, deserilaize_email_option, Email,
        Handle, Password, User,
    },
    storage::{patch_user, upsert_user},
};

#[tracing::instrument(
    name = "Updating user details",
    skip(user_id, user_details, db_pool),
    fields(
        id = %user_id,
        email = %user_details.email().as_ref(),
        handle = %user_details.handle().as_ref(),
        name = %user_details.name().unwrap_or_default(),
        profile_photo = %user_details.profile_photo().unwrap_or_default(),
        bio = %user_details.bio().unwrap_or_default(),
    )
)]
pub async fn update(
    user_id: Path<Uuid>,
    mut user_details: Json<User>,
    db_pool: Data<PgPool>,
) -> HttpResponse {
    let id = user_id.into_inner();
    user_details.set_id(id);
    match upsert_user(db_pool.get_ref(), &user_details).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().body("User not found"),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct PatchUserRequestBody {
    #[serde(deserialize_with = "deserilaize_email_option")]
    pub email: Option<Email>,
    #[serde(deserialize_with = "deserialize_handle_option")]
    pub handle: Option<Handle>,
    #[serde(deserialize_with = "deserialize_password_option")]
    pub password: Option<Password>,
    pub name: Option<String>,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
}

impl PartialEq for PatchUserRequestBody {
    fn eq(&self, other: &Self) -> bool {
        let email = self
            .email
            .clone()
            .and(other.email.clone())
            .map(|_| self.email == other.email)
            .unwrap_or(true);
        let handle = self
            .handle
            .clone()
            .and(other.handle.clone())
            .map(|_| self.handle == other.handle)
            .unwrap_or(true);
        let password = self
            .password
            .clone()
            .and(other.password.clone())
            .map(|_| self.password == other.password)
            .unwrap_or(true);
        let name = self
            .name
            .clone()
            .and(other.name.clone())
            .map(|_| self.name == other.name)
            .unwrap_or(true);
        let profile_photo = self
            .profile_photo
            .clone()
            .and(other.profile_photo.clone())
            .map(|_| self.profile_photo == other.profile_photo)
            .unwrap_or(true);
        let bio = self
            .bio
            .clone()
            .and(other.bio.clone().map(|_| self.bio == other.bio))
            .unwrap_or(true);
        email && handle && password && name && profile_photo && bio
    }
}

pub async fn patch(
    user_id: Path<Uuid>,
    user_details: Json<PatchUserRequestBody>,
    db_pool: Data<PgPool>,
) -> HttpResponse {
    let id = user_id.into_inner();
    match user_details.build_query(id) {
        Some(q) => match patch_user(&db_pool, q).await {
            Ok(_) => {
                tracing::info!("User {} successfully patched in database", id);
                HttpResponse::Ok().finish()
            }
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    tracing::error!("User {} not found for patch", id);
                    HttpResponse::NotFound().body("User not found")
                }
                other => {
                    tracing::error!("Failed to patch user {} in database: {:?}", id, other);
                    HttpResponse::InternalServerError().finish()
                }
            },
        },
        None => HttpResponse::NotModified().finish(),
    }
}

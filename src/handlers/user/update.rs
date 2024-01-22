use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::user::User, storage::upsert_user};

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

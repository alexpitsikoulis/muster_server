use actix_web::web::Json;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

use super::AsServer;
use crate::handlers::server::UpdateServerRequestData;

#[derive(Debug)]
pub struct ServerUpdate {
    id: Uuid,
    name: Option<String>,
    owner_id: Option<Uuid>,
    description: Option<String>,
    photo: Option<String>,
    cover_photo: Option<String>,
}

impl std::fmt::Display for ServerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ServerUpdate {
    pub fn from_request(
        body: &Json<UpdateServerRequestData>,
        id: Uuid,
    ) -> Result<Self, uuid::Error> {
        let owner_id = match body.owner_id.clone() {
            Some(owner_id) => match Uuid::from_str(&owner_id) {
                Ok(owner_id) => Some(owner_id),
                Err(e) => return Err(e),
            },
            None => None,
        };

        Ok(ServerUpdate {
            id,
            name: body.name.clone(),
            owner_id,
            description: body.description.clone(),
            photo: body.photo.clone(),
            cover_photo: body.cover_photo.clone(),
        })
    }
}

impl AsServer for ServerUpdate {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        match self.name.clone() {
            Some(name) => name,
            None => String::new(),
        }
    }

    fn owner_id(&self) -> Uuid {
        match self.owner_id {
            Some(id) => id,
            None => Uuid::nil(),
        }
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn photo(&self) -> Option<String> {
        self.photo.clone()
    }

    fn cover_photo(&self) -> Option<String> {
        self.cover_photo.clone()
    }

    fn created_at(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn updated_at(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        None
    }
}

use chrono::{Utc, DateTime};
use uuid::Uuid;
use crate::handlers::server::CreateServerRequestDataWithOwner;

#[derive(Debug)]
pub struct Server {
    id: Uuid,
    name: String,
    owner_id: Uuid,
    description: Option<String>,
    photo: Option<String>,
    cover_photo: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Server {
    pub fn new(
        id: Uuid,
        name: String,
        owner_id: Uuid,
        description: Option<String>,
        photo: Option<String>,
        cover_photo: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Server {
            id,
            name,
            owner_id,
            description,
            photo,
            cover_photo,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn photo(&self) -> Option<String> {
        self.photo.clone()
    }

    pub fn cover_photo(&self) -> Option<String> {
        self.cover_photo.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_owner_id(&mut self, owner_id: Uuid) {
        self.owner_id = owner_id
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description
    }

    pub fn set_photo(&mut self, photo: Option<String>) {
        self.photo = photo
    }

    pub fn set_cover_photo(&mut self, cover_photo: Option<String>) {
        self.cover_photo = cover_photo;
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at
    }

    pub fn set_deleted_at(&mut self, deleted_at: Option<DateTime<Utc>>) {
        self.deleted_at = deleted_at
    }
}

impl Into<Server> for CreateServerRequestDataWithOwner {
    fn into(self) -> Server {
        let now = Utc::now();
        Server::new(
            Uuid::new_v4(),
            self.data.name.clone(),
            self.owner_id,
            self.data.description.clone(),
            self.data.photo.clone(),
            self.data.cover_photo.clone(),
            now,
            now,
            None,
        )
    }
}
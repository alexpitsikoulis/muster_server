mod update;

pub use update::*;

use crate::handlers::server::CreateServerRequestDataWithOwner;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait AsServer {
    fn id(&self) -> Uuid;
    fn name(&self) -> String;
    fn owner_id(&self) -> Uuid;
    fn description(&self) -> Option<String>;
    fn photo(&self) -> Option<String>;
    fn cover_photo(&self) -> Option<String>;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
}

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

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl AsServer for Server {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn owner_id(&self) -> Uuid {
        self.owner_id
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
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
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

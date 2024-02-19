use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct DirectMessage {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    thread_id: Uuid,
    sender_id: Uuid,
    message: String,
    is_read: bool,
    reaction: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl PartialEq for DirectMessage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.thread_id == other.thread_id
            && self.sender_id == other.sender_id
            && self.message == other.message
            && self.is_read == other.is_read
            && self.reaction == other.reaction
            && self.deleted_at == other.deleted_at
    }
}

impl std::fmt::Display for DirectMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DirectMessage {
    pub fn new(
        id: Uuid,
        thread_id: Uuid,
        sender_id: Uuid,
        message: String,
        is_read: bool,
        reaction: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        DirectMessage {
            id,
            thread_id,
            sender_id,
            message,
            is_read,
            reaction,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn thread_id(&self) -> Uuid {
        self.thread_id
    }

    pub fn sender_id(&self) -> Uuid {
        self.sender_id
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    pub fn reaction(&self) -> Option<String> {
        self.reaction.clone()
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

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at
    }

    pub fn set_deleted_at(&mut self, deleted_at: Option<DateTime<Utc>>) {
        self.deleted_at = deleted_at
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct DMThread {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    first_user_id: Uuid,
    second_user_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl PartialEq for DMThread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.first_user_id == other.first_user_id
            && self.second_user_id == other.second_user_id
    }
}

impl std::fmt::Display for DMThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DMThread {
    pub fn new(
        id: Uuid,
        first_user_id: Uuid,
        second_user_id: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        DMThread {
            id,
            first_user_id,
            second_user_id,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn first_user_id(&self) -> Uuid {
        self.first_user_id
    }

    pub fn second_user_id(&self) -> Uuid {
        self.second_user_id
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

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at
    }

    pub fn set_deleted_at(&mut self, deleted_at: Option<DateTime<Utc>>) {
        self.deleted_at = deleted_at
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Email, Handle, User};

#[derive(Deserialize, Serialize)]
pub struct GetUserResponse {
    id: Uuid,
    email: Email,
    handle: Handle,
    name: Option<String>,
    profile_photo: Option<String>,
    bio: Option<String>,
}

impl GetUserResponse {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn profile_photo(&self) -> Option<String> {
        self.profile_photo.clone()
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }
}

impl From<User> for GetUserResponse {
    fn from(user: User) -> Self {
        GetUserResponse {
            id: user.id,
            email: user.email,
            handle: user.handle,
            name: user.name,
            profile_photo: user.profile_photo,
            bio: user.bio,
        }
    }
}

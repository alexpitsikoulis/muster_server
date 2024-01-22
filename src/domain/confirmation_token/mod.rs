mod tests;

use secrecy::{ExposeSecret, Secret};
use sqlx::Decode;
use uuid::Uuid;

#[derive(Decode)]
pub struct ConfirmationToken {
    inner: ConfirmationTokenInner,
    user_id: Uuid,
}

pub struct ConfirmationTokenInner(Secret<String>);

impl ConfirmationTokenInner {
    pub fn new(token: Secret<String>) -> Self {
        ConfirmationTokenInner(token)
    }
}

impl ConfirmationToken {
    pub fn new(inner: Secret<String>, user_id: Uuid) -> Self {
        ConfirmationToken {
            inner: ConfirmationTokenInner(inner),
            user_id,
        }
    }

    pub fn expose(&self) -> String {
        self.inner.0.expose_secret().clone()
    }

    pub fn inner(&self) -> Secret<String> {
        self.inner.0.clone()
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn set_inner(&mut self, inner: Secret<String>) {
        self.inner = ConfirmationTokenInner(inner);
    }

    pub fn set_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }
}

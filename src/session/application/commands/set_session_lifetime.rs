use chrono::{DateTime, Utc};

use crate::{session::domain::SessionId, user::domain::UserId};

use crate::{
    session::application::ports::{SessionReader, SessionWriter},
    shared::application::ApplicationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionLifetime {
    Short,
    Long,
}

pub struct SetSessionLifetime<R> {
    repository: R,
}

impl<R: SessionReader + SessionWriter> SetSessionLifetime<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        id: SessionId,
        lifetime: SessionLifetime,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<(), ApplicationError> {
        let session = self
            .repository
            .load(owner, id)
            .await?
            .ok_or(ApplicationError::NotFound("session"))?;
        let updated = match lifetime {
            SessionLifetime::Long => session.upgrade(now, expires_at)?,
            SessionLifetime::Short => session.downgrade(now, expires_at)?,
        };
        self.repository.save(&updated).await
    }
}

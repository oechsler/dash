use chrono::{DateTime, Utc};

use crate::{session::application::ports::SessionWriter, shared::application::ApplicationError};

pub struct CleanupExpiredSessions<R> {
    repository: R,
}

impl<R: SessionWriter> CleanupExpiredSessions<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, now: DateTime<Utc>) -> Result<(), ApplicationError> {
        self.repository.delete_expired(now).await
    }
}

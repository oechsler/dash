use chrono::{DateTime, Utc};

use crate::{
    session::{
        application::ports::{SessionReader, SessionWriter},
        domain::{SessionId, SessionMetadata},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct TouchSession<R> {
    repository: R,
}

impl<R: SessionReader + SessionWriter> TouchSession<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        id: SessionId,
        metadata: SessionMetadata,
        now: DateTime<Utc>,
    ) -> Result<(), ApplicationError> {
        let mut session = self
            .repository
            .load(owner, id)
            .await?
            .ok_or(ApplicationError::NotFound("session"))?;
        session.record_access(metadata, now)?;
        self.repository.save(&session).await
    }
}

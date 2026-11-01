use crate::shared::application::ApplicationResult;
use chrono::{DateTime, Utc};

use crate::{session::domain::SessionId, shared::domain::UserId};

use crate::{
    session::application::ports::{SessionReader, SessionWriter},
    shared::application::ApplicationError,
};

pub struct TerminateSession<R> {
    repository: R,
}

impl<R: SessionReader + SessionWriter> TerminateSession<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        id: SessionId,
        now: DateTime<Utc>,
    ) -> ApplicationResult<()> {
        let mut session = self
            .repository
            .load(owner, id)
            .await?
            .ok_or(ApplicationError::NotFound("session"))?;
        session.terminate(now);
        self.repository
            .save(&session)
            .await
            .map_err(ApplicationError::from)
    }
}

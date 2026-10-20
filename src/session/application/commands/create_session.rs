use chrono::{DateTime, Utc};

use crate::{
    session::{
        application::ports::SessionWriter,
        domain::{Session, SessionId, SessionMetadata, ShortLived, UserSession},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct CreateSessionInput {
    pub owner: UserId,
    pub metadata: SessionMetadata,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct CreateSession<R> {
    repository: R,
}

impl<R: SessionWriter> CreateSession<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: CreateSessionInput) -> Result<SessionId, ApplicationError> {
        let session = Session::<ShortLived>::new(
            input.owner,
            input.metadata,
            input.issued_at,
            input.expires_at,
        )?;
        let id = session.id();
        self.repository
            .save(&UserSession::ShortLived(session))
            .await?;
        Ok(id)
    }
}

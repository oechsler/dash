use chrono::{DateTime, Utc};

use crate::{
    session::{
        application::ports::SessionReader,
        domain::{SessionId, SessionMetadata, UserSession},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct SessionOverviewInput {
    pub owner: UserId,
    pub current_session_id: Option<SessionId>,
    pub current_metadata: SessionMetadata,
    pub now: DateTime<Utc>,
}

pub struct SessionOverviewItem {
    pub id: SessionId,
    pub metadata: SessionMetadata,
    pub issued_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub is_current: bool,
    pub is_long_lived: bool,
}

pub struct SessionOverview {
    pub sessions: Vec<SessionOverviewItem>,
}

pub struct GetSessionsOverview<R> {
    repository: R,
}

impl<R: SessionReader> GetSessionsOverview<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: SessionOverviewInput,
    ) -> Result<SessionOverview, ApplicationError> {
        let sessions = self.repository.list_for(&input.owner).await?;
        let mut items =
            Vec::with_capacity(sessions.len() + usize::from(input.current_session_id.is_some()));
        if let Some(current_id) = input.current_session_id {
            if let Some(current) = sessions.iter().find(|session| session.id() == current_id) {
                items.push(to_item(current, input.now, true));
            } else {
                items.push(SessionOverviewItem {
                    id: current_id,
                    metadata: input.current_metadata,
                    issued_at: input.now,
                    last_accessed_at: input.now,
                    expires_at: input.now,
                    is_active: true,
                    is_current: true,
                    is_long_lived: false,
                });
            }
        }
        items.extend(
            sessions
                .iter()
                .filter(|session| Some(session.id()) != input.current_session_id)
                .map(|session| to_item(session, input.now, false)),
        );
        Ok(SessionOverview { sessions: items })
    }
}

fn to_item(session: &UserSession, now: DateTime<Utc>, is_current: bool) -> SessionOverviewItem {
    SessionOverviewItem {
        id: session.id(),
        metadata: session.metadata().clone(),
        issued_at: session.issued_at(),
        last_accessed_at: session.last_accessed_at(),
        expires_at: session.expires_at(),
        is_active: session.is_active_at(now),
        is_current,
        is_long_lived: session.is_long_lived(),
    }
}

use chrono::{DateTime, Utc};

use crate::user::domain::UserId;

use super::{LongLived, Session, SessionError, SessionId, SessionMetadata, ShortLived};

/// Runtime representation for sessions loaded from storage or listed for a user.
#[derive(Debug, PartialEq, Eq)]
pub enum UserSession {
    ShortLived(Session<ShortLived>),
    LongLived(Session<LongLived>),
}

impl UserSession {
    pub fn id(&self) -> SessionId {
        match self {
            Self::ShortLived(session) => session.id(),
            Self::LongLived(session) => session.id(),
        }
    }

    pub fn is_long_lived(&self) -> bool {
        matches!(self, Self::LongLived(_))
    }

    pub fn owner(&self) -> &UserId {
        match self {
            Self::ShortLived(session) => session.owner(),
            Self::LongLived(session) => session.owner(),
        }
    }

    pub fn metadata(&self) -> &SessionMetadata {
        match self {
            Self::ShortLived(session) => session.metadata(),
            Self::LongLived(session) => session.metadata(),
        }
    }

    pub fn is_active_at(&self, now: DateTime<Utc>) -> bool {
        match self {
            Self::ShortLived(session) => session.is_active_at(now),
            Self::LongLived(session) => session.is_active_at(now),
        }
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        match self {
            Self::ShortLived(session) => session.expires_at(),
            Self::LongLived(session) => session.expires_at(),
        }
    }

    pub fn issued_at(&self) -> DateTime<Utc> {
        match self {
            Self::ShortLived(session) => session.issued_at(),
            Self::LongLived(session) => session.issued_at(),
        }
    }

    pub fn last_accessed_at(&self) -> DateTime<Utc> {
        match self {
            Self::ShortLived(session) => session.last_accessed_at(),
            Self::LongLived(session) => session.last_accessed_at(),
        }
    }

    pub fn terminated_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::ShortLived(session) => session.terminated_at(),
            Self::LongLived(session) => session.terminated_at(),
        }
    }

    pub fn record_access(
        &mut self,
        metadata: SessionMetadata,
        accessed_at: DateTime<Utc>,
    ) -> Result<(), SessionError> {
        match self {
            Self::ShortLived(session) => session.record_access(metadata, accessed_at),
            Self::LongLived(session) => session.record_access(metadata, accessed_at),
        }
    }

    pub fn refresh_expiry(
        &mut self,
        expires_at: DateTime<Utc>,
        refreshed_at: DateTime<Utc>,
    ) -> Result<(), SessionError> {
        match self {
            Self::ShortLived(session) => session.refresh_expiry(expires_at, refreshed_at),
            Self::LongLived(session) => session.refresh_expiry(expires_at, refreshed_at),
        }
    }

    pub fn terminate(&mut self, terminated_at: DateTime<Utc>) {
        match self {
            Self::ShortLived(session) => session.terminate(terminated_at),
            Self::LongLived(session) => session.terminate(terminated_at),
        }
    }

    pub fn upgrade(
        self,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        match self {
            Self::ShortLived(session) => session.upgrade(now, expires_at).map(Self::LongLived),
            long_lived @ Self::LongLived(_) => Ok(long_lived),
        }
    }

    pub fn downgrade(
        self,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        match self {
            Self::ShortLived(short_lived) => Ok(Self::ShortLived(short_lived)),
            Self::LongLived(session) => session.downgrade(now, expires_at).map(Self::ShortLived),
        }
    }
}

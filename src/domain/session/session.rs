use std::marker::PhantomData;

use chrono::{DateTime, Utc};

use crate::domain::user::UserId;

use super::{LongLived, SessionError, SessionId, SessionMetadata, ShortLived};

#[derive(Debug, PartialEq, Eq)]
struct SessionData {
    id: SessionId,
    owner: UserId,
    metadata: SessionMetadata,
    issued_at: DateTime<Utc>,
    last_accessed_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    terminated_at: Option<DateTime<Utc>>,
}

/// A session whose lifetime policy is encoded in its type parameter.
#[derive(Debug, PartialEq, Eq)]
pub struct Session<Lifetime> {
    data: SessionData,
    lifetime: PhantomData<Lifetime>,
}

impl<Lifetime> Session<Lifetime> {
    fn create(
        owner: UserId,
        metadata: SessionMetadata,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        if expires_at <= issued_at {
            return Err(SessionError::ExpiryNotAfterIssue {
                issued_at,
                expires_at,
            });
        }

        Ok(Self {
            data: SessionData {
                id: SessionId::new(),
                owner,
                metadata,
                issued_at,
                last_accessed_at: issued_at,
                expires_at,
                terminated_at: None,
            },
            lifetime: PhantomData,
        })
    }

    pub fn id(&self) -> SessionId {
        self.data.id
    }

    pub fn owner(&self) -> &UserId {
        &self.data.owner
    }

    pub fn metadata(&self) -> &SessionMetadata {
        &self.data.metadata
    }

    pub fn issued_at(&self) -> DateTime<Utc> {
        self.data.issued_at
    }

    pub fn last_accessed_at(&self) -> DateTime<Utc> {
        self.data.last_accessed_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.data.expires_at
    }

    pub fn terminated_at(&self) -> Option<DateTime<Utc>> {
        self.data.terminated_at
    }

    pub fn is_active_at(&self, now: DateTime<Utc>) -> bool {
        self.data.terminated_at.is_none() && now < self.data.expires_at
    }

    pub fn record_access(
        &mut self,
        metadata: SessionMetadata,
        accessed_at: DateTime<Utc>,
    ) -> Result<(), SessionError> {
        if !self.is_active_at(accessed_at) {
            return Err(SessionError::InactiveSession { accessed_at });
        }
        self.data.metadata = metadata;
        self.data.last_accessed_at = accessed_at;
        Ok(())
    }

    /// Marks the session as terminated. The calling use case scopes access by owner.
    pub fn terminate(&mut self, terminated_at: DateTime<Utc>) {
        self.data.terminated_at.get_or_insert(terminated_at);
    }

    fn transition<Next>(
        self,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session<Next>, SessionError> {
        if !self.is_active_at(now) {
            return Err(SessionError::CannotChangeInactiveSession);
        }
        if expires_at <= now {
            return Err(SessionError::ExpiryNotInFuture { expires_at, now });
        }

        Ok(Session {
            data: SessionData {
                expires_at,
                ..self.data
            },
            lifetime: PhantomData,
        })
    }
}

impl Session<ShortLived> {
    pub fn new(
        owner: UserId,
        metadata: SessionMetadata,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        Self::create(owner, metadata, issued_at, expires_at)
    }

    /// Consumes a short-lived session and returns its long-lived form.
    pub fn upgrade(
        self,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session<LongLived>, SessionError> {
        self.transition(now, expires_at)
    }
}

impl Session<LongLived> {
    pub fn new(
        owner: UserId,
        metadata: SessionMetadata,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        Self::create(owner, metadata, issued_at, expires_at)
    }

    /// Consumes a long-lived session and returns its short-lived form.
    pub fn downgrade(
        self,
        now: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session<ShortLived>, SessionError> {
        self.transition(now, expires_at)
    }
}

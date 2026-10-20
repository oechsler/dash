use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("session expiry {expires_at} must be after issue time {issued_at}")]
    ExpiryNotAfterIssue {
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },

    #[error("cannot change lifetime of an inactive session")]
    CannotChangeInactiveSession,

    #[error("cannot record access on an inactive session at {accessed_at}")]
    InactiveSession { accessed_at: DateTime<Utc> },

    #[error("new expiry {expires_at} must be after current time {now}")]
    ExpiryNotInFuture {
        expires_at: DateTime<Utc>,
        now: DateTime<Utc>,
    },
}

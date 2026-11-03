use crate::{
    session::{
        application::{
            commands::TerminateSession,
            ports::{SessionReader, SessionWriter},
        },
        domain::SessionId,
    },
    shared::{application::ApplicationResult, domain::UserId},
};
use chrono::Utc;

/// Handles `POST /api/v1/sessions/{session_id}/terminate`.
pub async fn handle<R>(sessions: R, owner: &UserId, id: SessionId) -> ApplicationResult<()>
where
    R: SessionReader + SessionWriter,
{
    TerminateSession::new(sessions)
        .execute(owner, id, Utc::now())
        .await
}

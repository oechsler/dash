use crate::{
    session::{
        application::{
            commands::InvalidateSession,
            ports::{SessionReader, SessionWriter},
        },
        domain::SessionId,
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `DELETE /api/v1/sessions/{session_id}`.
pub async fn handle<R>(sessions: R, owner: &UserId, id: SessionId) -> ApplicationResult<()>
where
    R: SessionReader + SessionWriter,
{
    InvalidateSession::new(sessions).execute(owner, id).await
}

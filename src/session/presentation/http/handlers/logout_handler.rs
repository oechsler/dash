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

/// Handles `POST /api/v1/sessions/logout` by invalidating the active session.
pub async fn handle<R>(
    sessions: R,
    owner: &UserId,
    current_session: SessionId,
) -> ApplicationResult<()>
where
    R: SessionReader + SessionWriter,
{
    InvalidateSession::new(sessions)
        .execute(owner, current_session)
        .await
}

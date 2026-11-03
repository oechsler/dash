use crate::{
    session::{
        application::{
            commands::{SessionLifetime, SetSessionLifetime},
            ports::{SessionReader, SessionWriter},
        },
        domain::SessionId,
    },
    shared::{application::ApplicationResult, domain::UserId},
};
use chrono::{Duration, Utc};

/// Handles `POST /api/v1/sessions/{session_id}/upgrade`.
pub async fn handle<R>(
    sessions: R,
    owner: &UserId,
    id: SessionId,
    long_lived_duration: Duration,
) -> ApplicationResult<()>
where
    R: SessionReader + SessionWriter,
{
    let now = Utc::now();
    SetSessionLifetime::new(sessions)
        .execute(
            owner,
            id,
            SessionLifetime::Long,
            now,
            now + long_lived_duration,
        )
        .await
}

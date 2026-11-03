use crate::{
    session::{
        application::{ports::SessionReader, queries::ListSessions},
        domain::UserSession,
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `GET /api/v1/sessions`.
pub async fn handle<R: SessionReader>(
    sessions: R,
    owner: &UserId,
) -> ApplicationResult<Vec<UserSession>> {
    ListSessions::new(sessions).execute(owner).await
}

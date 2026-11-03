use crate::{
    shared::{application::ApplicationResult, domain::UserId},
    theme::{
        application::{ports::ThemeReader, queries::ListThemes},
        domain::Theme,
    },
};

/// Handles `GET /api/v1/themes`.
pub async fn handle<R: ThemeReader>(
    repository: R,
    owner: &UserId,
) -> ApplicationResult<Vec<Theme>> {
    ListThemes::new(repository).execute(owner).await
}

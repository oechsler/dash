use crate::{
    shared::{application::ApplicationResult, domain::UserId},
    theme::{
        application::{ports::ThemeReader, queries::GetTheme},
        domain::{Theme, ThemeId},
    },
};

/// Handles `GET /api/v1/themes/{theme_id}`.
pub async fn handle<R: ThemeReader>(
    repository: R,
    owner: &UserId,
    id: ThemeId,
) -> ApplicationResult<Theme> {
    GetTheme::new(repository).execute(owner, id).await
}

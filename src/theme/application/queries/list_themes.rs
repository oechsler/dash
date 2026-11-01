use crate::{
    shared::application::ApplicationError,
    shared::domain::UserId,
    theme::{application::ports::ThemeReader, domain::Theme},
};

pub struct ListThemes<R> {
    repository: R,
}

impl<R: ThemeReader> ListThemes<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> Result<Vec<Theme>, ApplicationError> {
        self.repository
            .list_for(owner)
            .await
            .map_err(ApplicationError::from)
    }
}

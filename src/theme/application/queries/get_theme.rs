use crate::{
    shared::application::ApplicationError,
    theme::{
        application::ports::ThemeReader,
        domain::{Theme, ThemeId},
    },
    user::domain::UserId,
};

pub struct GetTheme<R> {
    repository: R,
}

impl<R: ThemeReader> GetTheme<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId, id: ThemeId) -> Result<Theme, ApplicationError> {
        self.repository
            .get_for(owner, id)
            .await?
            .ok_or(ApplicationError::NotFound("theme"))
    }
}

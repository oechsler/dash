use crate::{
    preferences::domain::UserPreferences, shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::preferences::application::ports::PreferencesReader;

pub struct GetPreferences<R> {
    repository: R,
}

impl<R: PreferencesReader> GetPreferences<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> Result<UserPreferences, ApplicationError> {
        Ok(self
            .repository
            .load(owner)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(owner.clone())))
    }
}

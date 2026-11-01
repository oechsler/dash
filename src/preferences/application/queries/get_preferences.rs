use crate::shared::application::ApplicationResult;
use crate::{preferences::domain::UserPreferences, shared::domain::UserId};

use crate::preferences::application::ports::PreferencesReader;

pub struct GetPreferences<R> {
    repository: R,
}

impl<R: PreferencesReader> GetPreferences<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> ApplicationResult<UserPreferences> {
        Ok(self
            .repository
            .load(owner)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(owner.clone())))
    }
}

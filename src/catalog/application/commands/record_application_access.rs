use std::collections::HashSet;

use crate::{
    catalog::{application::ports::ApplicationReader, domain::ApplicationId},
    preferences::application::ports::{PreferencesReader, PreferencesWriter},
    shared::application::ApplicationError,
    user::domain::{GroupId, UserId},
};

pub struct RecordApplicationAccess<A, P> {
    applications: A,
    preferences: P,
}

impl<A, P> RecordApplicationAccess<A, P>
where
    A: ApplicationReader,
    P: PreferencesReader + PreferencesWriter,
{
    pub fn new(applications: A, preferences: P) -> Self {
        Self {
            applications,
            preferences,
        }
    }

    pub async fn execute(
        &self,
        user_id: &UserId,
        groups: &HashSet<GroupId>,
        application_id: ApplicationId,
    ) -> Result<(), ApplicationError> {
        let is_visible = self
            .applications
            .get(application_id)
            .await?
            .is_some_and(|application| application.is_visible_to(groups));
        if !is_visible {
            return Err(ApplicationError::NotFound("application"));
        }
        let mut preferences = self.preferences.load(user_id).await?.unwrap_or_else(|| {
            crate::preferences::domain::UserPreferences::for_user(user_id.clone())
        });
        preferences.record_application_access(application_id);
        self.preferences.save(&preferences).await
    }
}

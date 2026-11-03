use std::collections::HashSet;

use crate::{
    catalog::{
        application::{commands::RecordApplicationAccess, ports::ApplicationReader},
        domain::ApplicationId,
    },
    preferences::application::ports::{PreferencesReader, PreferencesWriter},
    shared::{
        application::ApplicationResult,
        domain::{GroupId, UserId},
    },
};

/// Handles `POST /api/v1/applications/{application_id}/access`.
pub async fn handle<A, P>(
    applications: A,
    preferences: P,
    user: &UserId,
    groups: &HashSet<GroupId>,
    id: ApplicationId,
) -> ApplicationResult<()>
where
    A: ApplicationReader,
    P: PreferencesReader + PreferencesWriter,
{
    RecordApplicationAccess::new(applications, preferences)
        .execute(user, groups, id)
        .await
}

mod applications;
mod dashboard;
mod preferences;
mod themes;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

use super::model::GoExportV1;

pub(super) fn into_snapshot(
    owner: &UserId,
    document: GoExportV1,
) -> Result<UserDataSnapshot, ApplicationError> {
    let (themes, theme_ids) = themes::build(owner, document.themes)?;
    let preferences = preferences::build(owner, document.settings, &themes, &theme_ids)?;
    let dashboard = dashboard::build(owner, document.categories)?;
    let applications = applications::build(document.applications)?;
    Ok(UserDataSnapshot {
        dashboard: Some(dashboard),
        preferences: Some(preferences),
        themes,
        applications,
    })
}

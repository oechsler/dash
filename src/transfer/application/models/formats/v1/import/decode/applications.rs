use crate::{
    catalog::domain::{Application, ApplicationVisibility},
    dashboard::domain::WebAddress,
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::GroupId,
};

use super::super::model::GoApplicationV1;

pub(super) fn build(
    records: Option<Vec<GoApplicationV1>>,
) -> Result<Vec<Application>, ApplicationError> {
    records
        .unwrap_or_default()
        .into_iter()
        .map(|record| {
            let groups = record
                .visible_to_groups
                .into_iter()
                .map(GroupId::new)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Application::create(
                record.display_name,
                IconReference::parse(record.icon)?,
                WebAddress::parse(record.url)?,
                ApplicationVisibility::groups(groups),
            )?)
        })
        .collect()
}

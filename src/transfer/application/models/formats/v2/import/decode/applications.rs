use crate::{
    catalog::domain::{Application, ApplicationVisibility},
    dashboard::domain::WebAddress,
    icon::domain::IconReference,
    shared::application::ApplicationError,
    transfer::application::models::formats::v2::ApplicationExport,
    user::domain::GroupId,
};

pub(super) fn build(records: Vec<ApplicationExport>) -> Result<Vec<Application>, ApplicationError> {
    records
        .into_iter()
        .map(|record| {
            let groups = record
                .visible_to_groups
                .into_iter()
                .map(GroupId::new)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Application::create(
                record.name,
                IconReference::parse(record.icon)?,
                WebAddress::parse(record.address)?,
                ApplicationVisibility::groups(groups),
            )?)
        })
        .collect()
}

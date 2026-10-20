use crate::{
    catalog::domain::Application,
    shared::application::ApplicationError,
    transfer::application::models::{UserDataSnapshot, formats::content_hash},
};

pub(super) fn merge(
    target: &mut UserDataSnapshot,
    imported: &UserDataSnapshot,
) -> Result<(), ApplicationError> {
    let mut known: Vec<_> = target.applications.iter().map(application_hash).collect();
    for application in &imported.applications {
        let hash = application_hash(application);
        if !known.contains(&hash) {
            target.applications.push(application.clone());
            known.push(hash);
        }
    }
    Ok(())
}

fn application_hash(application: &Application) -> String {
    let mut groups: Vec<_> = application
        .visibility()
        .visible_to_groups()
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.as_str().to_owned())
                .collect()
        })
        .unwrap_or_default();
    groups.sort();
    let joined = groups.join(",");
    content_hash(&[
        application.icon().as_str(),
        application.name(),
        application.address().as_str(),
        &joined,
    ])
}

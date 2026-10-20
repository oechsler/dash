use crate::{
    catalog::domain::Application,
    transfer::application::models::formats::{content_hash, v2::ApplicationExport},
};

pub(super) fn export(applications: &[Application]) -> Vec<ApplicationExport> {
    applications.iter().map(export_application).collect()
}

fn export_application(application: &Application) -> ApplicationExport {
    let mut visible_to_groups: Vec<_> = application
        .visibility()
        .visible_to_groups()
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.as_str().to_owned())
                .collect()
        })
        .unwrap_or_default();
    visible_to_groups.sort();
    let joined = visible_to_groups.join(",");
    let icon = application.icon().as_str().to_owned();
    let address = application.address().as_str().to_owned();
    let name = application.name().to_owned();
    let hash = content_hash(&[&icon, &name, &address, &joined]);
    ApplicationExport {
        hash,
        name,
        icon,
        address,
        visible_to_groups,
    }
}

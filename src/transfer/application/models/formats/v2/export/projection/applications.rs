use crate::{
    catalog::domain::Application,
    transfer::application::models::formats::{
        application_groups, application_hash, v2::ApplicationExport,
    },
};

pub(super) fn export(applications: &[Application]) -> Vec<ApplicationExport> {
    applications.iter().map(export_application).collect()
}

fn export_application(application: &Application) -> ApplicationExport {
    let mut visible_to_groups = application_groups(application);
    visible_to_groups.sort();
    let icon = application.icon().as_str().to_owned();
    let address = application.address().as_str().to_owned();
    let name = application.name().to_owned();
    let hash = application_hash(application);
    ApplicationExport {
        hash,
        name,
        icon,
        address,
        visible_to_groups,
    }
}

use crate::{
    shared::application::ApplicationError,
    transfer::application::models::{UserDataSnapshot, formats::application_hash},
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

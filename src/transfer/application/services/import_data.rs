use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

mod applications;
mod dashboard;
mod preferences;
mod themes;

pub struct ImportDataService;

impl ImportDataService {
    pub fn apply(
        &self,
        snapshot: &mut UserDataSnapshot,
        owner: &UserId,
        imported: UserDataSnapshot,
        is_admin: bool,
    ) -> Result<(), ApplicationError> {
        let theme_ids = themes::merge(snapshot, &imported)?;
        dashboard::merge(snapshot, &imported, owner)?;
        if is_admin {
            applications::merge(snapshot, &imported)?;
        }
        preferences::merge(snapshot, owner, &imported, &theme_ids)?;
        Ok(())
    }
}

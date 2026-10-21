mod projection;
#[cfg(test)]
mod tests;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub(super) fn export(
    snapshot: UserDataSnapshot,
    owner: &UserId,
    username: String,
    include_applications: bool,
) -> Result<Vec<u8>, ApplicationError> {
    let document = projection::snapshot::build(snapshot, owner, username, include_applications)?;
    Ok(serde_json::to_vec(&document)?)
}

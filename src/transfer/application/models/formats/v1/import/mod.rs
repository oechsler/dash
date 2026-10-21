mod decode;
mod model;
mod signature;
#[cfg(test)]
mod tests;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub(super) fn import(payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
    let document = signature::parse(payload)?;
    decode::into_snapshot(owner, document)
}

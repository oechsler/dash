use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub trait ImportFormat: Send + Sync {
    fn version(&self) -> u32;
    fn decode(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError>;
}

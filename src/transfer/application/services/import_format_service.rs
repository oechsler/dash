use crate::{
    shared::application::ApplicationError,
    transfer::application::{error::TransferError, models::UserDataSnapshot},
    user::domain::UserId,
};

use crate::transfer::application::models::formats::{TransferFormat, V1Format, V2Format};

pub struct ImportFormatService {
    formats: Vec<Box<dyn TransferFormat>>,
}

impl ImportFormatService {
    pub fn new(formats: Vec<Box<dyn TransferFormat>>) -> Self {
        Self { formats }
    }

    pub fn supported_versions() -> Self {
        Self::new(vec![Box::new(V1Format), Box::new(V2Format)])
    }

    pub fn import(
        &self,
        payload: &[u8],
        owner: &UserId,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        let envelope: serde_json::Value = serde_json::from_slice(payload)?;
        let version = envelope
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .and_then(|version| u32::try_from(version).ok())
            .ok_or_else(|| {
                ApplicationError::from(TransferError::InvalidDocument(
                    "missing or invalid version".into(),
                ))
            })?;
        self.formats
            .iter()
            .find(|format| format.version() == version)
            .ok_or(TransferError::UnsupportedVersion(version))?
            .import(payload, owner)
    }
}

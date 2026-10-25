use crate::{
    shared::application::ApplicationError,
    transfer::{
        application::{TransferError, models::UserDataSnapshot},
        presentation::formats::{TransferFormat, V1Format, V2Format},
    },
    user::domain::UserId,
};

/// Selects and invokes the adapter matching an imported document's version.
pub struct ImportFormatRegistry {
    formats: Vec<Box<dyn TransferFormat>>,
}

impl ImportFormatRegistry {
    pub fn supported_versions() -> Self {
        Self {
            formats: vec![Box::new(V1Format), Box::new(V2Format)],
        }
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

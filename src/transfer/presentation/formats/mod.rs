mod import_format_registry;
#[cfg(test)]
mod import_format_registry_test;
mod signature_hash;
mod transfer_format;
mod v1;
pub mod v2;

macro_rules! impl_transfer_format {
    ($format:ident, $version:literal) => {
        pub struct $format;

        impl crate::transfer::presentation::formats::TransferFormat for $format {
            fn version(&self) -> u32 {
                $version
            }

            fn import(
                &self,
                payload: &[u8],
                owner: &crate::shared::domain::UserId,
            ) -> Result<
                crate::transfer::application::models::UserDataSnapshot,
                crate::shared::application::ApplicationError,
            > {
                super::import_service::ImportService.import(payload, owner)
            }

            fn export(
                &self,
                snapshot: crate::transfer::application::models::UserDataSnapshot,
                owner: &crate::shared::domain::UserId,
                username: String,
                include_applications: bool,
            ) -> Result<Vec<u8>, crate::shared::application::ApplicationError> {
                super::export_service::ExportService.export(
                    snapshot,
                    owner,
                    username,
                    include_applications,
                )
            }
        }
    };
}
pub(crate) use impl_transfer_format;

use crate::{
    dashboard::domain::Bookmark, preferences::domain::Language,
    shared::application::ApplicationError, transfer::application::TransferError,
};

fn import_document<Document>(
    payload: &[u8],
    owner: &crate::shared::domain::UserId,
    parse: impl FnOnce(&[u8]) -> Result<Document, ApplicationError>,
    into_snapshot: impl FnOnce(
        &crate::shared::domain::UserId,
        Document,
    ) -> Result<
        crate::transfer::application::models::UserDataSnapshot,
        ApplicationError,
    >,
) -> Result<crate::transfer::application::models::UserDataSnapshot, ApplicationError> {
    into_snapshot(owner, parse(payload)?)
}

pub(super) fn parse_language(value: &str) -> Result<Language, ApplicationError> {
    if value.is_empty() {
        return Ok(Language::Auto);
    }
    Language::try_from(value)
        .map_err(|_| TransferError::InvalidDocument(format!("unknown language `{value}`")).into())
}

pub(super) fn record_bookmark_access(
    bookmark: &mut Bookmark,
    timestamp: Option<String>,
) -> Result<(), ApplicationError> {
    let Some(timestamp) = timestamp else {
        return Ok(());
    };
    let accessed_at = chrono::DateTime::parse_from_rfc3339(&timestamp)
        .map_err(|error| {
            TransferError::InvalidDocument(format!("invalid bookmark last_accessed_at: {error}"))
        })?
        .with_timezone(&chrono::Utc);
    bookmark.record_access(accessed_at);
    Ok(())
}

pub use import_format_registry::ImportFormatRegistry;
pub(crate) use signature_hash::sha256_hex;
pub use transfer_format::TransferFormat;
pub use v1::V1Format;
pub use v2::V2Format;

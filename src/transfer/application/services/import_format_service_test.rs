use crate::{
    shared::application::ApplicationError,
    transfer::application::{error::TransferError, services::ImportFormatService},
};

#[test]
fn service_rejects_unregistered_transfer_versions() {
    let error = ImportFormatService::supported_versions()
        .import(
            br#"{"version":99}"#,
            &crate::user::domain::UserId::new("user").unwrap(),
        )
        .unwrap_err();

    assert!(matches!(
        error,
        ApplicationError::Transfer(TransferError::UnsupportedVersion(99))
    ));
}

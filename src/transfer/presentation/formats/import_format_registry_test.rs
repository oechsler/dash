use crate::{
    shared::application::ApplicationError, transfer::presentation::formats::ImportFormatRegistry,
    user::domain::UserId,
};

#[test]
fn registry_rejects_an_unregistered_version() {
    let payload = br#"{"version":99}"#;
    let error = ImportFormatRegistry::supported_versions()
        .import(payload, &UserId::new("user-1").unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        ApplicationError::Transfer(
            crate::transfer::application::TransferError::UnsupportedVersion(99)
        )
    ));
}

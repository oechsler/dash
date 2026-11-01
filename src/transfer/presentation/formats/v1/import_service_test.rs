use crate::{shared::application::ApplicationError, transfer::application::TransferError};

use super::import_service::ImportService;

#[test]
fn import_requires_a_signature() {
    let payload = br#"{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"sam","settings":{"language":"en","timezone":"auto"},"themes":[],"categories":[]}"#;

    assert!(matches!(
        ImportService.import(
            payload,
            &crate::shared::domain::UserId::try_from("user-1").unwrap()
        ),
        Err(ApplicationError::Transfer(TransferError::MissingSignature))
    ));
}

#[test]
fn imports_a_signed_v1_compatibility_fixture() {
    let payload = include_bytes!("../../../../../tests/fixtures/transfer-v1.json");
    let owner = crate::shared::domain::UserId::try_from("fixture-user").unwrap();

    let imported = ImportService.import(payload, &owner).unwrap();

    assert!(imported.dashboard.is_some());
    assert!(imported.preferences.is_some());
    assert!(imported.themes.is_empty());
    assert!(imported.applications.is_empty());
}

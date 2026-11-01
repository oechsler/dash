use crate::transfer::application::TransferError;

use super::{
    import_service::ImportService,
    transfer_model::{SettingsModel, UserDataModel},
};

fn document() -> UserDataModel {
    UserDataModel {
        version: 2,
        exported_at: "2026-01-01T00:00:00Z".into(),
        username: "sam".into(),
        settings: SettingsModel::default(),
        themes: Vec::new(),
        categories_sort_order: "user_defined".into(),
        categories: Vec::new(),
        applications_sort_order: None,
        applications: Vec::new(),
        signature: None,
    }
}

#[test]
fn signature_verifies_export_and_detects_changes() {
    let mut document = document();
    document.sign().unwrap();
    assert!(document.verify().unwrap());

    document.username = "changed".into();
    assert!(!document.verify().unwrap());
}

#[test]
fn import_requires_a_signature() {
    let payload = serde_json::to_vec(&document()).unwrap();
    assert!(matches!(
        ImportService.import(
            &payload,
            &crate::shared::domain::UserId::try_from("user-1").unwrap(),
        ),
        Err(crate::shared::application::ApplicationError::Transfer(
            TransferError::MissingSignature
        ))
    ));
}

#[test]
fn imports_a_signed_v2_compatibility_fixture() {
    let payload = include_bytes!("../../../../../tests/fixtures/transfer-v2.json");
    let owner = crate::shared::domain::UserId::try_from("fixture-user").unwrap();

    let imported = ImportService.import(payload, &owner).unwrap();

    assert!(imported.dashboard.is_some());
    assert!(imported.preferences.is_some());
    assert!(imported.themes.is_empty());
    assert!(imported.applications.is_empty());
}

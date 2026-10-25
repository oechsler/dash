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
        application_sort_order: None,
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
            &crate::user::domain::UserId::new("user-1").unwrap(),
        ),
        Err(crate::shared::application::ApplicationError::Transfer(
            TransferError::MissingSignature
        ))
    ));
}

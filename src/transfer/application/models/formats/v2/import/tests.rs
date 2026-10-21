use crate::transfer::application::error::TransferError;

use super::{
    model::{SettingsExport, UserDataExport},
    signature,
};

fn document() -> UserDataExport {
    UserDataExport {
        version: 2,
        exported_at: "2026-01-01T00:00:00Z".into(),
        username: "sam".into(),
        settings: SettingsExport::default(),
        themes: Vec::new(),
        categories: Vec::new(),
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
        signature::parse(&payload),
        Err(crate::shared::application::ApplicationError::Transfer(
            TransferError::MissingSignature
        ))
    ));
}

use super::{SettingsExport, UserDataExport};

fn export() -> UserDataExport {
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
    let mut export = export();
    export.sign().unwrap();
    assert!(export.verify().unwrap());

    export.username = "changed".into();
    assert!(!export.verify().unwrap());
}

#[test]
fn export_requires_a_signature() {
    assert!(!export().verify().unwrap());
}

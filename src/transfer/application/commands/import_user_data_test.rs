use crate::{
    dashboard::domain::Category, shared::application::ApplicationError, theme::domain::ThemeMode,
    transfer::application::models::UserDataSnapshot, user::domain::UserId,
};
use sha2::{Digest, Sha256};

use super::import_user_data_test_support::{MemoryUserData, command, export, input, raw_input};

#[tokio::test]
async fn import_merges_idempotently_and_only_imports_catalog_for_admins() {
    let repository = MemoryUserData::default();
    let command = command(&repository);
    let owner = UserId::new("target-user").unwrap();

    command
        .execute(input(owner.clone(), false, export()))
        .await
        .unwrap();
    command
        .execute(input(owner, false, export()))
        .await
        .unwrap();

    let snapshot = repository.snapshot.lock().unwrap().clone();
    let dashboard = snapshot.dashboard.unwrap();
    assert_eq!(dashboard.categories().len(), 1);
    assert_eq!(dashboard.categories()[0].bookmarks().len(), 1);
    assert_eq!(snapshot.themes.len(), 1);
    assert!(snapshot.applications.is_empty());
    let preferences = snapshot.preferences.unwrap();
    assert_eq!(preferences.language().as_str(), "de");
    assert!(preferences.theme_id(ThemeMode::Light).is_some());
    assert_eq!(*repository.saves.lock().unwrap(), 2);
}

#[tokio::test]
async fn import_adds_bookmarks_to_an_existing_matching_category() {
    let repository = MemoryUserData::default();
    let owner = UserId::new("target-user").unwrap();
    let mut snapshot = UserDataSnapshot::default();
    let mut dashboard = crate::dashboard::domain::UserDashboard::new(owner.clone());
    dashboard.add_category(Category::new("Work").unwrap());
    snapshot.dashboard = Some(dashboard);
    *repository.snapshot.lock().unwrap() = snapshot;

    command(&repository)
        .execute(input(owner, false, export()))
        .await
        .unwrap();

    let snapshot = repository.snapshot.lock().unwrap().clone();
    let dashboard = snapshot.dashboard.unwrap();
    assert_eq!(dashboard.categories().len(), 1);
    assert_eq!(dashboard.categories()[0].bookmarks().len(), 1);
}

#[tokio::test]
async fn admin_imports_shared_catalog_applications() {
    let repository = MemoryUserData::default();

    command(&repository)
        .execute(input(UserId::new("admin").unwrap(), true, export()))
        .await
        .unwrap();

    assert_eq!(repository.snapshot.lock().unwrap().applications.len(), 1);
}

#[tokio::test]
async fn import_rejects_a_tampered_payload_before_writing() {
    let repository = MemoryUserData::default();
    let mut data = export();
    data.username = "tampered".into();

    let error = command(&repository)
        .execute(input(UserId::new("target-user").unwrap(), false, data))
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        ApplicationError::Transfer(
            crate::transfer::application::error::TransferError::InvalidSignature
        )
    ));
    assert_eq!(*repository.saves.lock().unwrap(), 0);
}

#[tokio::test]
async fn command_imports_current_go_v1_backup_into_the_snapshot() {
    let canonical = r##"{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"legacy \u003cuser\u003e","settings":{"language":"en","timezone":"auto","theme_name":"Soft"},"themes":[{"hash":"old-theme-hash","name":"Soft","primary":"#111111","secondary":"#222222","tertiary":"#333333"}],"categories":[{"hash":"old-category-hash","display_name":"Work","is_shelved":false,"bookmarks":[{"hash":"old-bookmark-hash","icon":"mdi:book","display_name":"Docs","url":"https://docs.example"}]}]}"##;
    let digest: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
    let signature = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let payload = format!(
        r##"{{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"legacy <user>","settings":{{"language":"en","timezone":"auto","theme_name":"Soft"}},"themes":[{{"hash":"old-theme-hash","name":"Soft","primary":"#111111","secondary":"#222222","tertiary":"#333333"}}],"categories":[{{"hash":"old-category-hash","display_name":"Work","is_shelved":false,"bookmarks":[{{"hash":"old-bookmark-hash","icon":"mdi:book","display_name":"Docs","url":"https://docs.example"}}]}}],"signature":"{signature}"}}"##
    );
    let repository = MemoryUserData::default();

    command(&repository)
        .execute(raw_input(
            UserId::new("target-user").unwrap(),
            false,
            payload.into_bytes(),
        ))
        .await
        .unwrap();

    let snapshot = repository.snapshot.lock().unwrap().clone();
    assert_eq!(
        snapshot.dashboard.unwrap().categories()[0]
            .bookmarks()
            .len(),
        1
    );
    assert_eq!(snapshot.themes[0].mode(), ThemeMode::Light);
    assert!(
        snapshot
            .preferences
            .unwrap()
            .theme_id(ThemeMode::Light)
            .is_some()
    );
}

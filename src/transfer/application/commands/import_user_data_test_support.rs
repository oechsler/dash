use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    transfer::{
        application::commands::{ImportUserData, ImportUserDataInput},
        application::{
            models::UserDataSnapshot,
            ports::{UserDataSnapshotReader, UserDataSnapshotWriter},
            services::MergeUserDataService,
        },
        presentation::formats::v2::{
            ApplicationModel, BookmarkModel, CategoryModel, SettingsModel, ThemeModel,
            UserDataModel,
        },
    },
    user::domain::UserId,
};

#[derive(Clone, Default)]
pub(super) struct MemoryUserData {
    pub snapshot: Arc<Mutex<UserDataSnapshot>>,
    pub saves: Arc<Mutex<usize>>,
}

#[async_trait]
impl UserDataSnapshotReader for MemoryUserData {
    async fn load(&self, _owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        Ok(self.snapshot.lock().unwrap().clone())
    }
}

#[async_trait]
impl UserDataSnapshotWriter for MemoryUserData {
    async fn save(
        &self,
        _owner: &UserId,
        snapshot: &UserDataSnapshot,
    ) -> Result<(), ApplicationError> {
        *self.snapshot.lock().unwrap() = snapshot.clone();
        *self.saves.lock().unwrap() += 1;
        Ok(())
    }
}

pub(super) fn export() -> UserDataModel {
    let mut export = UserDataModel {
        version: 2,
        exported_at: "2026-01-01T00:00:00Z".into(),
        username: "source-user".into(),
        settings: SettingsModel {
            language: "de".into(),
            time_zone: "Europe/Berlin".into(),
            light_theme: Some("Soft".into()),
            dark_theme: None,
        },
        themes: vec![ThemeModel {
            hash: "ignored-client-hash".into(),
            name: "Soft".into(),
            mode: "light".into(),
            primary: "#111111".into(),
            secondary: "#222222".into(),
            tertiary: "#333333".into(),
        }],
        categories_sort_order: "user_defined".into(),
        categories: vec![CategoryModel {
            hash: "ignored-category-hash".into(),
            name: "Work".into(),
            shelved: false,
            bookmarks_sort_order: "user_defined".into(),
            bookmarks: vec![BookmarkModel {
                hash: "ignored-bookmark-hash".into(),
                name: "Docs".into(),
                icon: "mdi:book".into(),
                address: "https://docs.example".into(),
                last_accessed_at: None,
            }],
        }],
        application_sort_order: Some("user_defined".into()),
        applications: vec![ApplicationModel {
            hash: "ignored-app-hash".into(),
            name: "Portal".into(),
            icon: "mdi:portal".into(),
            address: "https://portal.example".into(),
            visible_to_groups: vec!["staff".into()],
            health_check: None,
        }],
        signature: None,
    };
    export.sign().unwrap();
    export
}

pub(super) fn input(owner: UserId, is_admin: bool, data: UserDataModel) -> ImportUserDataInput {
    let payload = serde_json::to_vec(&data).unwrap();
    let snapshot =
        crate::transfer::presentation::formats::ImportFormatRegistry::supported_versions()
            .import(&payload, &owner)
            .unwrap();
    ImportUserDataInput {
        owner,
        is_admin,
        snapshot,
    }
}

pub(super) fn raw_input(owner: UserId, is_admin: bool, payload: Vec<u8>) -> ImportUserDataInput {
    let snapshot =
        crate::transfer::presentation::formats::ImportFormatRegistry::supported_versions()
            .import(&payload, &owner)
            .unwrap_or_default();
    ImportUserDataInput {
        owner,
        is_admin,
        snapshot,
    }
}

pub(super) fn command(
    repository: &MemoryUserData,
) -> ImportUserData<MemoryUserData, MemoryUserData> {
    ImportUserData::new(repository.clone(), repository.clone(), MergeUserDataService)
}

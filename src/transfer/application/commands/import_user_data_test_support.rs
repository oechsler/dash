use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    transfer::{
        application::models::formats::v2::{
            ApplicationExport, BookmarkExport, CategoryExport, SettingsExport, ThemeExport,
            UserDataExport,
        },
        application::{
            commands::{ImportUserData, ImportUserDataInput},
            services::{ImportDataService, ImportFormatService},
        },
        application::{
            models::UserDataSnapshot,
            ports::{UserDataReader, UserDataWriter},
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
impl UserDataReader for MemoryUserData {
    async fn load(&self, _owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        Ok(self.snapshot.lock().unwrap().clone())
    }
}

#[async_trait]
impl UserDataWriter for MemoryUserData {
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

pub(super) fn export() -> UserDataExport {
    let mut export = UserDataExport {
        version: 2,
        exported_at: "2026-01-01T00:00:00Z".into(),
        username: "source-user".into(),
        settings: SettingsExport {
            language: "de".into(),
            time_zone: "Europe/Berlin".into(),
            light_theme: Some("Soft".into()),
            dark_theme: None,
        },
        themes: vec![ThemeExport {
            hash: "ignored-client-hash".into(),
            name: "Soft".into(),
            mode: "light".into(),
            primary: "#111111".into(),
            secondary: "#222222".into(),
            tertiary: "#333333".into(),
        }],
        categories: vec![CategoryExport {
            hash: "ignored-category-hash".into(),
            name: "Work".into(),
            shelved: false,
            bookmarks: vec![BookmarkExport {
                hash: "ignored-bookmark-hash".into(),
                name: "Docs".into(),
                icon: "mdi:book".into(),
                address: "https://docs.example".into(),
            }],
        }],
        applications: vec![ApplicationExport {
            hash: "ignored-app-hash".into(),
            name: "Portal".into(),
            icon: "mdi:portal".into(),
            address: "https://portal.example".into(),
            visible_to_groups: vec!["staff".into()],
        }],
        signature: None,
    };
    export.sign().unwrap();
    export
}

pub(super) fn input(owner: UserId, is_admin: bool, data: UserDataExport) -> ImportUserDataInput {
    ImportUserDataInput {
        owner,
        is_admin,
        payload: serde_json::to_vec(&data).unwrap(),
    }
}

pub(super) fn raw_input(owner: UserId, is_admin: bool, payload: Vec<u8>) -> ImportUserDataInput {
    ImportUserDataInput {
        owner,
        is_admin,
        payload,
    }
}

pub(super) fn command(
    repository: &MemoryUserData,
) -> ImportUserData<MemoryUserData, MemoryUserData> {
    ImportUserData::new(
        repository.clone(),
        repository.clone(),
        ImportFormatService::supported_versions(),
        ImportDataService,
    )
}

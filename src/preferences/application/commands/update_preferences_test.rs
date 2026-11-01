use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    preferences::{
        application::{
            commands::{UpdatePreferences, UpdatePreferencesInput},
            ports::{PreferencesReader, PreferencesWriter},
        },
        domain::{Language, UserPreferences},
    },
    shared::application::{ApplicationError, ApplicationPortError},
    shared::domain::UserId,
    theme::{
        application::ports::ThemeReader,
        domain::{Theme, ThemeId},
    },
};

struct PreferencesRepository {
    saved: Arc<AtomicBool>,
}

#[async_trait]
impl PreferencesReader for PreferencesRepository {
    async fn load(&self, _: &UserId) -> Result<Option<UserPreferences>, ApplicationPortError> {
        Ok(None)
    }
}

#[async_trait]
impl PreferencesWriter for PreferencesRepository {
    async fn save(&self, _: &UserPreferences) -> Result<(), ApplicationPortError> {
        self.saved.store(true, Ordering::SeqCst);
        Ok(())
    }
}

struct MissingThemeReader {
    requested_owner: Arc<Mutex<Option<UserId>>>,
}

#[async_trait]
impl ThemeReader for MissingThemeReader {
    async fn list_for(&self, _: &UserId) -> Result<Vec<Theme>, ApplicationPortError> {
        Ok(Vec::new())
    }

    async fn get_for(
        &self,
        owner: &UserId,
        _: ThemeId,
    ) -> Result<Option<Theme>, ApplicationPortError> {
        *self.requested_owner.lock().unwrap() = Some(owner.clone());
        Ok(None)
    }
}

#[tokio::test]
async fn update_rejects_a_theme_the_owner_cannot_resolve() {
    let owner = UserId::try_from("owner-1").unwrap();
    let requested_owner = Arc::new(Mutex::new(None));
    let saved = Arc::new(AtomicBool::new(false));
    let update = UpdatePreferences::new(
        PreferencesRepository {
            saved: saved.clone(),
        },
        MissingThemeReader {
            requested_owner: requested_owner.clone(),
        },
    );

    let result = update
        .execute(UpdatePreferencesInput {
            owner: owner.clone(),
            language: Language::English,
            time_zone: "auto".to_owned(),
            light_theme_id: None,
            dark_theme_id: Some(ThemeId::UserDefined(Uuid::new_v4())),
        })
        .await;

    assert!(matches!(result, Err(ApplicationError::NotFound("theme"))));
    assert_eq!(*requested_owner.lock().unwrap(), Some(owner));
    assert!(!saved.load(Ordering::SeqCst));
}

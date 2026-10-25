use std::collections::HashMap;

use sha2::{Digest, Sha256};

use crate::{
    catalog::domain::{Application, ApplicationVisibility},
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
    icon::domain::IconReference,
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::application::ApplicationError,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    transfer::{
        application::{TransferError, models::UserDataSnapshot},
        presentation::formats::sha256_hex,
    },
    user::domain::{GroupId, UserId},
};

use super::transfer_model::{GoApplicationV1, GoCategoryV1, GoExportV1, GoSettingsV1, GoThemeV1};

pub(super) struct ImportService;

impl ImportService {
    pub(super) fn import(
        &self,
        payload: &[u8],
        owner: &UserId,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        let document = Self::parse(payload)?;
        Self::into_snapshot(owner, document)
    }

    fn parse(payload: &[u8]) -> Result<GoExportV1, ApplicationError> {
        let mut document: GoExportV1 = serde_json::from_slice(payload)?;
        let signature = document
            .signature
            .take()
            .ok_or(TransferError::MissingSignature)?;
        let canonical = Self::go_json(serde_json::to_string(&document)?);
        let digest: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
        if sha256_hex(&digest) != signature {
            return Err(TransferError::InvalidSignature.into());
        }
        Ok(document)
    }

    fn go_json(json: String) -> String {
        json.replace('&', "\\u0026")
            .replace('<', "\\u003c")
            .replace('>', "\\u003e")
            .replace('\u{2028}', "\\u2028")
            .replace('\u{2029}', "\\u2029")
    }

    fn into_snapshot(
        owner: &UserId,
        document: GoExportV1,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        let (themes, theme_ids) = Self::build_themes(owner, document.themes)?;
        let preferences = Self::build_preferences(owner, document.settings, &themes, &theme_ids)?;
        let dashboard = Self::build_dashboard(owner, document.categories)?;
        let applications = Self::build_applications(document.applications)?;
        Ok(UserDataSnapshot {
            dashboard: Some(dashboard),
            preferences: Some(preferences),
            themes,
            applications,
        })
    }

    fn build_applications(
        records: Option<Vec<GoApplicationV1>>,
    ) -> Result<Vec<Application>, ApplicationError> {
        records
            .unwrap_or_default()
            .into_iter()
            .map(|record| {
                let groups = record
                    .visible_to_groups
                    .into_iter()
                    .map(GroupId::new)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Application::create(
                    record.display_name,
                    IconReference::parse(record.icon)?,
                    WebAddress::parse(record.url)?,
                    ApplicationVisibility::groups(groups),
                )?)
            })
            .collect()
    }

    fn build_dashboard(
        owner: &UserId,
        records: Vec<GoCategoryV1>,
    ) -> Result<UserDashboard, ApplicationError> {
        let mut dashboard = UserDashboard::new(owner.clone());
        for record in records {
            let mut category = Category::new(record.display_name)?;
            for bookmark in record.bookmarks {
                category.add_bookmark(Bookmark::new(
                    bookmark.display_name,
                    IconReference::parse(bookmark.icon)?,
                    WebAddress::parse(bookmark.url)?,
                )?);
            }
            if record.is_shelved {
                dashboard.add_category(ShelvedCategory::from(category));
            } else {
                dashboard.add_category(category);
            }
        }
        Ok(dashboard)
    }

    fn build_preferences(
        owner: &UserId,
        settings: GoSettingsV1,
        themes: &[Theme],
        theme_ids: &HashMap<String, ThemeId>,
    ) -> Result<UserPreferences, ApplicationError> {
        let mut preferences = UserPreferences::for_user(owner.clone());
        preferences.set_language(Self::parse_language(&settings.language)?);
        preferences.set_time_zone(TimeZone::parse(settings.timezone)?);
        if let Some(id) = settings
            .theme_name
            .and_then(|name| theme_ids.get(&name).copied())
        {
            let theme = themes
                .iter()
                .find(|theme| theme.id() == id)
                .ok_or(ApplicationError::NotFound("theme"))?;
            preferences.select_theme(theme)?;
        }
        Ok(preferences)
    }

    fn parse_language(value: &str) -> Result<Language, ApplicationError> {
        match value {
            "" | "auto" => Ok(Language::Auto),
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            _ => Err(TransferError::InvalidDocument(format!("unknown language `{value}`")).into()),
        }
    }

    fn build_themes(
        owner: &UserId,
        records: Vec<GoThemeV1>,
    ) -> Result<(Vec<Theme>, HashMap<String, ThemeId>), ApplicationError> {
        let themes = records
            .into_iter()
            .map(|record| {
                Ok(Theme::user_defined()
                    .owner(owner.clone())
                    .name(record.name)
                    .mode(ThemeMode::Light)
                    .palette(ThemePalette::new(
                        Color::parse(record.primary)?,
                        Color::parse(record.secondary)?,
                        Color::parse(record.tertiary)?,
                    ))
                    .call()?)
            })
            .collect::<Result<Vec<_>, ApplicationError>>()?;
        let theme_ids = themes
            .iter()
            .map(|theme| (theme.name().to_owned(), theme.id()))
            .collect();
        Ok((themes, theme_ids))
    }
}

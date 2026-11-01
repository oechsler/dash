use std::collections::HashMap;

use sha2::{Digest, Sha256};

use crate::{
    catalog::domain::{Application, ApplicationVisibility},
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
    icon::domain::IconReference,
    preferences::domain::{TimeZone, UserPreferences},
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    shared::domain::{GroupId, UserId},
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    transfer::{
        application::{TransferError, models::UserDataSnapshot},
        presentation::formats::{parse_language, record_bookmark_access, sha256_hex},
    },
};

use super::transfer_model::{GoApplicationV1, GoCategoryV1, GoExportV1, GoSettingsV1, GoThemeV1};

pub(super) struct ImportService;

impl ImportService {
    pub(super) fn import(
        &self,
        payload: &[u8],
        owner: &UserId,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        super::super::import_document(payload, owner, Self::parse, Self::into_snapshot)
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
            application_catalog_settings: None,
            imported_theme_names: None,
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
                    .map(GroupId::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Application::create(
                    record.display_name,
                    IconReference::try_from(record.icon)?,
                    WebAddress::try_from(record.url)?,
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
            let sort_order = Self::parse_bookmarks_sort_order(
                record
                    .bookmarks_sort_order
                    .as_deref()
                    .unwrap_or("alphabetical"),
            )?;
            let mut category = Category::new(record.display_name)?;
            category.set_bookmarks_sort_order(sort_order);
            for bookmark in record.bookmarks {
                let mut domain_bookmark = Bookmark::new(
                    bookmark.display_name,
                    IconReference::try_from(bookmark.icon)?,
                    WebAddress::try_from(bookmark.url)?,
                )?;
                record_bookmark_access(&mut domain_bookmark, bookmark.last_accessed_at)?;
                category.add_bookmark(domain_bookmark);
            }
            if record.is_shelved {
                dashboard.add_category(ShelvedCategory::from(category));
            } else {
                dashboard.add_category(category);
            }
        }
        Ok(dashboard)
    }

    fn parse_bookmarks_sort_order(value: &str) -> Result<CollectionSortOrder, ApplicationError> {
        CollectionSortOrder::try_from(value).map_err(|_| {
            TransferError::InvalidDocument(format!("unknown bookmark sort order `{value}`")).into()
        })
    }

    fn build_preferences(
        owner: &UserId,
        settings: GoSettingsV1,
        themes: &[Theme],
        theme_ids: &HashMap<String, ThemeId>,
    ) -> Result<UserPreferences, ApplicationError> {
        let mut preferences = UserPreferences::for_user(owner.clone());
        preferences.set_language(parse_language(&settings.language)?);
        preferences.set_time_zone(TimeZone::try_from(settings.timezone)?);
        if let Some(id) = settings
            .theme_name
            .and_then(|name| theme_ids.get(&name).cloned())
        {
            let theme = themes
                .iter()
                .find(|theme| theme.id() == id)
                .ok_or(ApplicationError::NotFound("theme"))?;
            preferences.select_theme(theme.mode(), theme.id());
        }
        Ok(preferences)
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
                        Color::try_from(record.primary)?,
                        Color::try_from(record.secondary)?,
                        Color::try_from(record.tertiary)?,
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

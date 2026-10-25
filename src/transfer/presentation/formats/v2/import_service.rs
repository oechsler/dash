use std::collections::HashMap;

use crate::{
    catalog::domain::{Application, ApplicationVisibility},
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
    icon::domain::IconReference,
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::application::ApplicationError,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    transfer::{
        application::{TransferError, models::UserDataSnapshot},
        presentation::formats::v2::*,
    },
    user::domain::{GroupId, UserId},
};

type ThemeIdByNameAndMode = HashMap<(String, ThemeMode), ThemeId>;

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

    fn parse(payload: &[u8]) -> Result<UserDataModel, ApplicationError> {
        let document: UserDataModel = serde_json::from_slice(payload)?;
        if document.signature.is_none() {
            return Err(TransferError::MissingSignature.into());
        }
        if !document.verify()? {
            return Err(TransferError::InvalidSignature.into());
        }
        Ok(document)
    }

    fn into_snapshot(
        owner: &UserId,
        document: UserDataModel,
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
        records: Vec<ApplicationModel>,
    ) -> Result<Vec<Application>, ApplicationError> {
        records
            .into_iter()
            .map(|record| {
                let groups = record
                    .visible_to_groups
                    .into_iter()
                    .map(GroupId::new)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Application::create(
                    record.name,
                    IconReference::parse(record.icon)?,
                    WebAddress::parse(record.address)?,
                    ApplicationVisibility::groups(groups),
                )?)
            })
            .collect()
    }

    fn build_dashboard(
        owner: &UserId,
        records: Vec<CategoryModel>,
    ) -> Result<UserDashboard, ApplicationError> {
        let mut dashboard = UserDashboard::new(owner.clone());
        for record in records {
            let mut category = Category::new(record.name)?;
            for bookmark in record.bookmarks {
                category.add_bookmark(Bookmark::new(
                    bookmark.name,
                    IconReference::parse(bookmark.icon)?,
                    WebAddress::parse(bookmark.address)?,
                )?);
            }
            if record.shelved {
                dashboard.add_category(ShelvedCategory::from(category));
            } else {
                dashboard.add_category(category);
            }
        }
        Ok(dashboard)
    }

    fn build_preferences(
        owner: &UserId,
        settings: SettingsModel,
        themes: &[Theme],
        theme_ids: &ThemeIdByNameAndMode,
    ) -> Result<UserPreferences, ApplicationError> {
        let mut preferences = UserPreferences::for_user(owner.clone());
        preferences.set_language(Self::parse_language(&settings.language)?);
        preferences.set_time_zone(TimeZone::parse(settings.time_zone)?);
        Self::select_theme(
            &mut preferences,
            ThemeMode::Light,
            settings.light_theme.as_deref(),
            themes,
            theme_ids,
        )?;
        Self::select_theme(
            &mut preferences,
            ThemeMode::Dark,
            settings.dark_theme.as_deref(),
            themes,
            theme_ids,
        )?;
        Ok(preferences)
    }

    fn select_theme(
        preferences: &mut UserPreferences,
        mode: ThemeMode,
        name: Option<&str>,
        themes: &[Theme],
        theme_ids: &ThemeIdByNameAndMode,
    ) -> Result<(), ApplicationError> {
        let Some(name) = name else { return Ok(()) };
        let Some(id) = theme_ids.get(&(name.to_owned(), mode)) else {
            return Ok(());
        };
        let theme = themes
            .iter()
            .find(|theme| theme.id() == *id)
            .ok_or(ApplicationError::NotFound("theme"))?;
        preferences.select_theme(theme)?;
        Ok(())
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
        records: Vec<ThemeModel>,
    ) -> Result<(Vec<Theme>, ThemeIdByNameAndMode), ApplicationError> {
        let themes = records
            .into_iter()
            .map(|record| {
                let mode = Self::parse_mode(&record.mode)?;
                Ok(Theme::user_defined()
                    .owner(owner.clone())
                    .name(record.name)
                    .mode(mode)
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
            .map(|theme| ((theme.name().to_owned(), theme.mode()), theme.id()))
            .collect();
        Ok((themes, theme_ids))
    }

    fn parse_mode(value: &str) -> Result<ThemeMode, ApplicationError> {
        match value {
            "light" => Ok(ThemeMode::Light),
            "dark" => Ok(ThemeMode::Dark),
            _ => {
                Err(TransferError::InvalidDocument(format!("unknown theme mode `{value}`")).into())
            }
        }
    }
}

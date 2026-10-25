use std::collections::HashMap;

use crate::{
    catalog::domain::{
        Application, ApplicationCatalogSettings, ApplicationHealthCheck, ApplicationVisibility,
        CatalogError, ExpectedStatusClass, HealthCheckUrl,
    },
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
    icon::domain::IconReference,
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    transfer::{
        application::{
            TransferError,
            models::{ImportedThemeNames, UserDataSnapshot},
        },
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
        let imported_theme_names = ImportedThemeNames {
            light: document.settings.light_theme.clone(),
            dark: document.settings.dark_theme.clone(),
        };
        let preferences = Self::build_preferences(owner, document.settings, &themes, &theme_ids)?;
        let dashboard =
            Self::build_dashboard(owner, document.categories, &document.categories_sort_order)?;
        let applications = Self::build_applications(document.applications)?;
        let application_catalog_settings = document
            .applications_sort_order
            .as_deref()
            .map(|sort_order| Self::build_application_catalog_settings(sort_order, &applications))
            .transpose()?;
        Ok(UserDataSnapshot {
            dashboard: Some(dashboard),
            preferences: Some(preferences),
            themes,
            applications,
            application_catalog_settings,
            imported_theme_names: Some(imported_theme_names),
        })
    }

    fn build_application_catalog_settings(
        sort_order: &str,
        applications: &[Application],
    ) -> Result<ApplicationCatalogSettings, ApplicationError> {
        let mut settings = ApplicationCatalogSettings::default();
        let sort_order = Self::parse_sort_order(sort_order)?;
        settings.set_sort_order(sort_order);
        if sort_order == CollectionSortOrder::UserDefined {
            let application_ids = applications.iter().map(Application::id).collect::<Vec<_>>();
            settings.set_manual_order(application_ids.clone(), &application_ids)?;
        }
        Ok(settings)
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
                let health_check = Self::parse_health_check(record.health_check)?;
                let mut application = Application::create(
                    record.name,
                    IconReference::parse(record.icon)?,
                    WebAddress::parse(record.address)?,
                    ApplicationVisibility::groups(groups),
                )?;
                application.set_health_check(health_check);
                Ok(application)
            })
            .collect()
    }

    fn parse_health_check(
        model: Option<HealthCheckModel>,
    ) -> Result<Option<ApplicationHealthCheck>, ApplicationError> {
        model
            .map(|model| {
                let url = model
                    .url
                    .map(HealthCheckUrl::parse)
                    .transpose()
                    .map_err(CatalogError::from)?;
                let expected_status_class = match model.expected_status_class.as_str() {
                    "2xx" => ExpectedStatusClass::Success2xx,
                    "3xx" => ExpectedStatusClass::Redirect3xx,
                    "4xx" => ExpectedStatusClass::ClientError4xx,
                    other => {
                        return Err(TransferError::InvalidDocument(format!(
                            "unknown expected health-check status class `{other}`"
                        ))
                        .into());
                    }
                };
                Ok(ApplicationHealthCheck::new(
                    url,
                    expected_status_class,
                    model.ignore_tls_errors,
                ))
            })
            .transpose()
    }

    fn build_dashboard(
        owner: &UserId,
        records: Vec<CategoryModel>,
        categories_sort_order: &str,
    ) -> Result<UserDashboard, ApplicationError> {
        let mut dashboard = UserDashboard::new(owner.clone());
        dashboard.set_categories_sort_order(Self::parse_sort_order(categories_sort_order)?);
        for record in records {
            let sort_order = Self::parse_sort_order(&record.bookmarks_sort_order)?;
            let mut category = Category::new(record.name)?;
            category.set_bookmarks_sort_order(sort_order);
            for bookmark in record.bookmarks {
                let mut domain_bookmark = Bookmark::new(
                    bookmark.name,
                    IconReference::parse(bookmark.icon)?,
                    WebAddress::parse(bookmark.address)?,
                )?;
                if let Some(last_accessed_at) = bookmark.last_accessed_at {
                    let last_accessed_at = chrono::DateTime::parse_from_rfc3339(&last_accessed_at)
                        .map_err(|error| {
                            TransferError::InvalidDocument(format!(
                                "invalid bookmark last_accessed_at: {error}"
                            ))
                        })?
                        .with_timezone(&chrono::Utc);
                    domain_bookmark.record_access(last_accessed_at);
                }
                category.add_bookmark(domain_bookmark);
            }
            if record.shelved {
                dashboard.add_category(ShelvedCategory::from(category));
            } else {
                dashboard.add_category(category);
            }
        }
        Ok(dashboard)
    }

    fn parse_sort_order(value: &str) -> Result<CollectionSortOrder, ApplicationError> {
        match value {
            "alphabetical" => Ok(CollectionSortOrder::Alphabetical),
            "recently_accessed" => Ok(CollectionSortOrder::RecentlyAccessed),
            "user_defined" => Ok(CollectionSortOrder::UserDefined),
            other => {
                Err(TransferError::InvalidDocument(format!("unknown sort order `{other}`")).into())
            }
        }
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

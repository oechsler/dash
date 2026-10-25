use crate::{
    catalog::domain::Application,
    dashboard::domain::{Bookmark, DashboardCategory},
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeIdentity, ThemeMode},
    transfer::{
        application::models::UserDataSnapshot,
        presentation::formats::v2::{
            ApplicationModel, BookmarkModel, CategoryModel, SettingsModel, ThemeModel,
            UserDataModel,
        },
    },
    user::domain::UserId,
};

pub(super) struct ExportService;

impl ExportService {
    pub(super) fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError> {
        let document = Self::build_document(snapshot, owner, username, include_applications)?;
        Ok(serde_json::to_vec(&document)?)
    }

    fn build_document(
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<UserDataModel, ApplicationError> {
        let preferences = snapshot
            .preferences
            .unwrap_or_else(|| UserPreferences::for_user(owner.clone()));
        let themes = snapshot
            .themes
            .iter()
            .filter(|theme| matches!(theme.identity(), ThemeIdentity::UserDefined { .. }))
            .map(Self::export_theme)
            .collect();
        let light_theme = preferences
            .theme_id(ThemeMode::Light)
            .and_then(|id| Self::theme_name(&snapshot.themes, id));
        let dark_theme = preferences
            .theme_id(ThemeMode::Dark)
            .and_then(|id| Self::theme_name(&snapshot.themes, id));
        let categories = snapshot
            .dashboard
            .as_ref()
            .map(|dashboard| Self::export_categories(dashboard.categories()))
            .unwrap_or_default();
        let applications = if include_applications {
            Self::export_applications(&snapshot.applications)
        } else {
            Vec::new()
        };

        let mut document = UserDataModel {
            version: 2,
            exported_at: chrono::Utc::now().to_rfc3339(),
            username,
            settings: SettingsModel {
                language: preferences.language().as_str().to_owned(),
                time_zone: preferences.time_zone().as_str().to_owned(),
                light_theme,
                dark_theme,
            },
            themes,
            categories,
            applications,
            signature: None,
        };
        document.sign()?;
        Ok(document)
    }

    fn export_applications(applications: &[Application]) -> Vec<ApplicationModel> {
        applications.iter().map(Self::export_application).collect()
    }

    fn export_application(application: &Application) -> ApplicationModel {
        let mut visible_to_groups = UserDataSnapshot::application_groups(application);
        visible_to_groups.sort();
        ApplicationModel {
            hash: UserDataSnapshot::application_hash(application),
            name: application.name().to_owned(),
            icon: application.icon().as_str().to_owned(),
            address: application.address().as_str().to_owned(),
            visible_to_groups,
        }
    }

    fn export_categories(categories: &[DashboardCategory]) -> Vec<CategoryModel> {
        categories.iter().map(Self::export_category).collect()
    }

    fn export_category(category: &DashboardCategory) -> CategoryModel {
        CategoryModel {
            bookmarks: category
                .bookmarks()
                .iter()
                .map(Self::export_bookmark)
                .collect(),
            hash: UserDataSnapshot::category_hash(category),
            name: category.name().to_owned(),
            shelved: matches!(category, DashboardCategory::Shelved(_)),
        }
    }

    fn export_bookmark(bookmark: &Bookmark) -> BookmarkModel {
        BookmarkModel {
            hash: UserDataSnapshot::bookmark_hash(bookmark),
            name: bookmark.name().to_owned(),
            icon: bookmark.icon().as_str().to_owned(),
            address: bookmark.address().as_str().to_owned(),
        }
    }

    fn theme_name(themes: &[Theme], id: crate::theme::domain::ThemeId) -> Option<String> {
        themes
            .iter()
            .find(|theme| theme.id() == id)
            .map(|theme| theme.name().to_owned())
    }

    fn export_theme(theme: &Theme) -> ThemeModel {
        let mode = match theme.mode() {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        ThemeModel {
            hash: UserDataSnapshot::theme_hash(theme),
            name: theme.name().to_owned(),
            mode: mode.to_owned(),
            primary: theme.palette().primary().as_str().to_owned(),
            secondary: theme.palette().secondary().as_str().to_owned(),
            tertiary: theme.palette().tertiary().as_str().to_owned(),
        }
    }
}

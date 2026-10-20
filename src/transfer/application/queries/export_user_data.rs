use crate::{
    catalog::domain::Application,
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId, ThemeMode, ThemeScope},
    transfer::application::{
        ApplicationExport, BookmarkExport, CategoryExport, SettingsExport, ThemeExport,
        UserDataExport, content_hash,
        ports::{UserDataReader, UserDataSnapshot},
    },
    user::domain::UserId,
};

pub struct ExportUserData<R> {
    repository: R,
}

impl<R: UserDataReader> ExportUserData<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<UserDataExport, ApplicationError> {
        let snapshot = self.repository.load(owner).await?;
        export_snapshot(snapshot, owner, username, include_applications)
    }
}

fn export_snapshot(
    snapshot: UserDataSnapshot,
    owner: &UserId,
    username: String,
    include_applications: bool,
) -> Result<UserDataExport, ApplicationError> {
    let preferences = snapshot
        .preferences
        .unwrap_or_else(|| UserPreferences::for_user(owner.clone()));
    let themes: Vec<_> = snapshot
        .themes
        .iter()
        .filter(|theme| matches!(theme.scope(), ThemeScope::UserDefined(_)))
        .map(export_theme)
        .collect();
    let light = preferences
        .theme_id(ThemeMode::Light)
        .and_then(|id| theme_name(&snapshot.themes, id));
    let dark = preferences
        .theme_id(ThemeMode::Dark)
        .and_then(|id| theme_name(&snapshot.themes, id));
    let categories = snapshot
        .dashboard
        .map(|dashboard| {
            dashboard
                .categories()
                .iter()
                .map(|category| {
                    let bookmarks = category
                        .bookmarks()
                        .iter()
                        .map(|bookmark| {
                            let icon = bookmark.icon().as_str().to_owned();
                            let address = bookmark.address().as_str().to_owned();
                            let name = bookmark.name().to_owned();
                            BookmarkExport {
                                hash: content_hash(&[&icon, &name, &address]),
                                name,
                                icon,
                                address,
                            }
                        })
                        .collect();
                    let name = category.name().to_owned();
                    let shelved = category.is_shelved();
                    let hash =
                        content_hash(&[name.as_str(), if shelved { "true" } else { "false" }]);
                    CategoryExport {
                        hash,
                        name,
                        shelved,
                        bookmarks,
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    let applications = if include_applications {
        snapshot
            .applications
            .iter()
            .map(export_application)
            .collect()
    } else {
        Vec::new()
    };
    let mut export = UserDataExport {
        version: 2,
        username,
        settings: SettingsExport {
            language: preferences.language().as_str().to_owned(),
            time_zone: preferences.time_zone().as_str().to_owned(),
            light_theme: light,
            dark_theme: dark,
        },
        themes,
        categories,
        applications,
        signature: None,
    };
    export.sign()?;
    Ok(export)
}

fn theme_name(themes: &[Theme], id: ThemeId) -> Option<String> {
    themes
        .iter()
        .find(|theme| theme.id() == id)
        .map(|theme| theme.name().to_owned())
}

fn export_theme(theme: &Theme) -> ThemeExport {
    let mode = match theme.mode() {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    }
    .to_owned();
    let name = theme.name().to_owned();
    let primary = theme.palette().primary().as_str().to_owned();
    let secondary = theme.palette().secondary().as_str().to_owned();
    let tertiary = theme.palette().tertiary().as_str().to_owned();
    let hash = content_hash(&[&name, &mode, &primary, &secondary, &tertiary]);
    ThemeExport {
        hash,
        name,
        mode,
        primary,
        secondary,
        tertiary,
    }
}

fn export_application(application: &Application) -> ApplicationExport {
    let mut groups: Vec<_> = application
        .visibility()
        .visible_to_groups()
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.as_str().to_owned())
                .collect()
        })
        .unwrap_or_default();
    groups.sort();
    let joined = groups.join(",");
    let icon = application.icon().as_str().to_owned();
    let address = application.address().as_str().to_owned();
    let name = application.name().to_owned();
    let hash = content_hash(&[&icon, &name, &address, &joined]);
    ApplicationExport {
        hash,
        name,
        icon,
        address,
        visible_to_groups: groups,
    }
}

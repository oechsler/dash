use sha2::{Digest, Sha256};

use crate::{
    catalog::domain::{Application, ApplicationCatalogSettings},
    dashboard::domain::{Bookmark, DashboardCategory},
    theme::domain::{Theme, ThemeMode},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportedThemeNames {
    pub light: Option<String>,
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UserDataSnapshot {
    pub dashboard: Option<crate::dashboard::domain::UserDashboard>,
    pub preferences: Option<crate::preferences::domain::UserPreferences>,
    pub themes: Vec<crate::theme::domain::Theme>,
    pub applications: Vec<crate::catalog::domain::Application>,
    pub application_catalog_settings: Option<ApplicationCatalogSettings>,
    pub imported_theme_names: Option<ImportedThemeNames>,
}

impl UserDataSnapshot {
    pub(crate) fn application_hash(application: &Application) -> String {
        let mut groups = Self::application_groups(application);
        groups.sort();
        let joined = groups.join(",");
        let health_check = application.health_check().map(|check| {
            [
                check
                    .url()
                    .map_or(application.address().as_str(), |url| url.as_str()),
                check.expected_status_class().as_str(),
                if check.ignore_tls_errors() {
                    "true"
                } else {
                    "false"
                },
            ]
            .join("|")
        });
        content_hash(&[
            application.icon().as_str(),
            application.name(),
            application.address().as_str(),
            &joined,
            health_check.as_deref().unwrap_or("disabled"),
        ])
    }

    pub(crate) fn application_groups(application: &Application) -> Vec<String> {
        application
            .visibility()
            .visible_to_groups()
            .map(|groups| {
                groups
                    .iter()
                    .map(|group| group.as_str().to_owned())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub(crate) fn category_hash(category: &DashboardCategory) -> String {
        content_hash(&[
            category.name(),
            if matches!(category, DashboardCategory::Shelved(_)) {
                "true"
            } else {
                "false"
            },
        ])
    }

    pub(crate) fn bookmark_hash(bookmark: &Bookmark) -> String {
        content_hash(&[
            bookmark.icon().as_str(),
            bookmark.name(),
            bookmark.address().as_str(),
        ])
    }

    pub(crate) fn theme_hash(theme: &Theme) -> String {
        let mode = match theme.mode() {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        content_hash(&[
            theme.name(),
            mode,
            theme.palette().primary().as_str(),
            theme.palette().secondary().as_str(),
            theme.palette().tertiary().as_str(),
        ])
    }
}

fn content_hash(parts: &[&str]) -> String {
    let joined = parts.join("\0");
    let digest: [u8; 32] = Sha256::digest(joined.as_bytes()).into();
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(32);
    for byte in &digest[..8] {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

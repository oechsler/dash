use sha2::{Digest, Sha256};

use crate::{
    catalog::domain::{Application, ApplicationCatalogSettings},
    dashboard::domain::{Bookmark, DashboardCategory, UserDashboard},
    preferences::domain::UserPreferences,
    theme::domain::{Theme, ThemeMode},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportedThemeNames {
    pub light: Option<String>,
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UserDataSnapshot {
    pub dashboard: Option<UserDashboard>,
    pub preferences: Option<UserPreferences>,
    pub themes: Vec<Theme>,
    pub applications: Vec<Application>,
    pub application_catalog_settings: Option<ApplicationCatalogSettings>,
    pub imported_theme_names: Option<ImportedThemeNames>,
}

impl UserDataSnapshot {
    pub(crate) fn application_hash(application: &Application) -> String {
        let mut groups = Self::application_groups(application);
        groups.sort();
        let mut parts = vec![
            application.icon().as_str().to_owned(),
            application.name().to_owned(),
            application.address().as_str().to_owned(),
            groups.len().to_string(),
        ];
        parts.extend(groups);
        if let Some(check) = application.health_check() {
            parts.extend([
                "enabled".to_owned(),
                check
                    .url()
                    .map_or(application.address().as_str(), |url| url.as_str())
                    .to_owned(),
                check.expected_status_class().as_str().to_owned(),
                check.ignore_tls_errors().to_string(),
            ]);
        } else {
            parts.push("disabled".to_owned());
        }
        content_hash(parts.iter().map(String::as_str))
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
        content_hash([
            category.name(),
            if matches!(category, DashboardCategory::Shelved(_)) {
                "true"
            } else {
                "false"
            },
        ])
    }

    pub(crate) fn bookmark_hash(bookmark: &Bookmark) -> String {
        content_hash([
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
        content_hash([
            theme.name(),
            mode,
            theme.palette().primary().as_str(),
            theme.palette().secondary().as_str(),
            theme.palette().tertiary().as_str(),
        ])
    }
}

fn content_hash<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    let digest = hasher.finalize();
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut hexadecimal = String::with_capacity(digest.len() * 2);
    for byte in digest {
        hexadecimal.push(HEX_DIGITS[usize::from(byte >> 4)] as char);
        hexadecimal.push(HEX_DIGITS[usize::from(byte & 0x0f)] as char);
    }
    hexadecimal
}

#[cfg(test)]
mod tests {
    use super::content_hash;

    #[test]
    fn content_hash_preserves_field_boundaries() {
        assert_ne!(content_hash(["ab", "c"]), content_hash(["a", "bc"]));
    }
}

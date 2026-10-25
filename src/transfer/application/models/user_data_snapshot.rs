use sha2::{Digest, Sha256};

use crate::{
    catalog::domain::Application,
    dashboard::domain::{Bookmark, DashboardCategory},
    theme::domain::{Theme, ThemeMode},
};

#[derive(Debug, Clone, Default)]
pub struct UserDataSnapshot {
    pub dashboard: Option<crate::dashboard::domain::UserDashboard>,
    pub preferences: Option<crate::preferences::domain::UserPreferences>,
    pub themes: Vec<crate::theme::domain::Theme>,
    pub applications: Vec<crate::catalog::domain::Application>,
}

impl UserDataSnapshot {
    pub(crate) fn application_hash(application: &Application) -> String {
        let mut groups = Self::application_groups(application);
        groups.sort();
        let joined = groups.join(",");
        content_hash(&[
            application.icon().as_str(),
            application.name(),
            application.address().as_str(),
            &joined,
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

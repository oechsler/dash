use crate::transfer::application::models::formats::v2::{
    ApplicationExport, BookmarkExport, CategoryExport, SettingsExport, ThemeExport, UserDataExport,
};

use super::super::model::GoExportV1;

impl GoExportV1 {
    pub(super) fn into_v2_export(self) -> UserDataExport {
        let _legacy_version = self.version;
        UserDataExport {
            version: 2,
            exported_at: self.exported_at,
            username: self.username,
            settings: SettingsExport {
                language: self.settings.language,
                time_zone: self.settings.timezone,
                light_theme: self.settings.theme_name,
                dark_theme: None,
            },
            themes: self
                .themes
                .into_iter()
                .map(|theme| ThemeExport {
                    hash: theme.hash,
                    name: theme.name,
                    mode: "light".into(),
                    primary: theme.primary,
                    secondary: theme.secondary,
                    tertiary: theme.tertiary,
                })
                .collect(),
            categories: self
                .categories
                .into_iter()
                .map(|category| CategoryExport {
                    hash: category.hash,
                    name: category.display_name,
                    shelved: category.is_shelved,
                    bookmarks: category
                        .bookmarks
                        .into_iter()
                        .map(|bookmark| BookmarkExport {
                            hash: bookmark.hash,
                            name: bookmark.display_name,
                            icon: bookmark.icon,
                            address: bookmark.url,
                        })
                        .collect(),
                })
                .collect(),
            applications: self
                .applications
                .unwrap_or_default()
                .into_iter()
                .map(|application| ApplicationExport {
                    hash: application.hash,
                    name: application.display_name,
                    icon: application.icon,
                    address: application.url,
                    visible_to_groups: application.visible_to_groups,
                })
                .collect(),
            signature: None,
        }
    }
}

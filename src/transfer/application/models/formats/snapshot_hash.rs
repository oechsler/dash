use crate::{
    catalog::domain::Application,
    dashboard::domain::{Bookmark, DashboardCategory},
    theme::domain::{Theme, ThemeMode},
    transfer::application::models::formats::content_hash,
};

pub(crate) fn application_hash(application: &Application) -> String {
    let mut groups = application_groups(application);
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

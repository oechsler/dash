use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoExportV1 {
    pub version: u32,
    pub exported_at: String,
    pub username: String,
    pub settings: GoSettingsV1,
    pub themes: Vec<GoThemeV1>,
    pub categories: Vec<GoCategoryV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<GoApplicationV1>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoSettingsV1 {
    pub language: String,
    pub timezone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoThemeV1 {
    pub hash: String,
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoCategoryV1 {
    pub hash: String,
    pub display_name: String,
    pub is_shelved: bool,
    pub bookmarks: Vec<GoBookmarkV1>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoBookmarkV1 {
    pub hash: String,
    pub icon: String,
    pub display_name: String,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct GoApplicationV1 {
    pub hash: String,
    pub icon: String,
    pub display_name: String,
    pub url: String,
    #[serde(default)]
    pub visible_to_groups: Vec<String>,
}

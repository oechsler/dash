use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transfer::presentation::formats::sha256_hex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDataModel {
    pub version: u32,
    pub exported_at: String,
    pub username: String,
    pub settings: SettingsModel,
    pub themes: Vec<ThemeModel>,
    pub categories_sort_order: String,
    pub categories: Vec<CategoryModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications_sort_order: Option<String>,
    #[serde(default)]
    pub applications: Vec<ApplicationModel>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsModel {
    pub language: String,
    pub time_zone: String,
    #[serde(default)]
    pub light_theme: Option<String>,
    #[serde(default)]
    pub dark_theme: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeModel {
    pub hash: String,
    pub name: String,
    pub mode: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryModel {
    pub hash: String,
    pub name: String,
    pub shelved: bool,
    #[serde(
        default = "default_bookmarks_sort_order",
        skip_serializing_if = "is_alphabetical_sort_order"
    )]
    pub bookmarks_sort_order: String,
    pub bookmarks: Vec<BookmarkModel>,
}

fn default_bookmarks_sort_order() -> String {
    "alphabetical".to_owned()
}

fn is_alphabetical_sort_order(value: &String) -> bool {
    value == "alphabetical"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookmarkModel {
    pub hash: String,
    pub name: String,
    pub icon: String,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationModel {
    pub hash: String,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub visible_to_groups: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheckModel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthCheckModel {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default = "default_expected_status_class")]
    pub expected_status_class: String,
    #[serde(default)]
    pub ignore_tls_errors: bool,
}

fn default_expected_status_class() -> String {
    "2xx".to_owned()
}

impl UserDataModel {
    pub fn sign(&mut self) -> Result<(), serde_json::Error> {
        self.signature = None;
        let content = serde_json::to_vec(self)?;
        let digest: [u8; 32] = Sha256::digest(content).into();
        self.signature = Some(sha256_hex(&digest));
        Ok(())
    }

    pub fn verify(&self) -> Result<bool, serde_json::Error> {
        let Some(signature) = &self.signature else {
            return Ok(false);
        };
        let mut unsigned = self.clone();
        unsigned.signature = None;
        let digest: [u8; 32] = Sha256::digest(serde_json::to_vec(&unsigned)?).into();
        Ok(sha256_hex(&digest) == *signature)
    }
}

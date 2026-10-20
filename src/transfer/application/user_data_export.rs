use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDataExport {
    pub version: u32,
    pub username: String,
    pub settings: SettingsExport,
    pub themes: Vec<ThemeExport>,
    pub categories: Vec<CategoryExport>,
    #[serde(default)]
    pub applications: Vec<ApplicationExport>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SettingsExport {
    pub language: String,
    pub time_zone: String,
    #[serde(default)]
    pub light_theme: Option<String>,
    #[serde(default)]
    pub dark_theme: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeExport {
    pub hash: String,
    pub name: String,
    pub mode: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryExport {
    pub hash: String,
    pub name: String,
    pub shelved: bool,
    pub bookmarks: Vec<BookmarkExport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookmarkExport {
    pub hash: String,
    pub name: String,
    pub icon: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationExport {
    pub hash: String,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub visible_to_groups: Vec<String>,
}

impl UserDataExport {
    pub fn sign(&mut self) -> Result<(), serde_json::Error> {
        self.signature = None;
        let content = serde_json::to_vec(self)?;
        let digest: [u8; 32] = Sha256::digest(content).into();
        self.signature = Some(hex(&digest));
        Ok(())
    }

    pub fn verify(&self) -> Result<bool, serde_json::Error> {
        let Some(signature) = &self.signature else {
            return Ok(false);
        };
        let mut unsigned = self.clone();
        unsigned.signature = None;
        let digest: [u8; 32] = Sha256::digest(serde_json::to_vec(&unsigned)?).into();
        Ok(hex(&digest) == *signature)
    }
}

pub fn content_hash(parts: &[&str]) -> String {
    let joined = parts.join("\0");
    let digest: [u8; 32] = Sha256::digest(joined.as_bytes()).into();
    hex(&digest)[..16].to_owned()
}

fn hex<const N: usize>(bytes: &[u8; N]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

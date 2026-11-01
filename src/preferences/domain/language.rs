#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    Auto,
    English,
    German,
}

/// Result for parsing supported language codes.
pub type LanguageResult<T> = Result<T, &'static str>;

impl Language {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::English => "en",
            Self::German => "de",
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> LanguageResult<Self> {
        match value {
            "auto" => Ok(Self::Auto),
            "en" => Ok(Self::English),
            "de" => Ok(Self::German),
            _ => Err("unknown language"),
        }
    }
}

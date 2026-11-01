use thiserror::Error;

use super::{ThemeId, ThemeIdError, color::ColorError, global_theme_id::GlobalThemeIdError};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThemeError {
    #[error("theme palette color is invalid: {0}")]
    Color(#[from] ColorError),

    #[error("theme name must not be empty (received {value:?})")]
    EmptyName { value: String },

    #[error("global theme ID is invalid: {0}")]
    GlobalId(#[from] GlobalThemeIdError),

    #[error("theme ID is invalid: {0}")]
    ThemeId(#[from] ThemeIdError),

    #[error("theme {theme_id:?} cannot be deleted")]
    NotDeletable { theme_id: ThemeId },
}

/// Result for operations that coordinate themes and their identifiers.
pub type DomainResult<T> = Result<T, ThemeError>;

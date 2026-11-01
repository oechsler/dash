use thiserror::Error;

use super::{ColorError, ThemeId};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThemeError {
    #[error("theme palette color is invalid: {0}")]
    Color(#[from] ColorError),

    #[error("theme name must not be empty (received {value:?})")]
    EmptyName { value: String },

    #[error("global theme ID must not be empty")]
    EmptyGlobalId,

    #[error("theme {theme_id:?} cannot be deleted")]
    NotDeletable { theme_id: ThemeId },
}

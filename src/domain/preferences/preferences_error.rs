use thiserror::Error;

use crate::domain::theme::ThemeId;
use crate::domain::user::UserId;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PreferencesError {
    #[error("theme {theme_id:?} is not available to user {user_id:?}")]
    ThemeNotAvailableToUser { theme_id: ThemeId, user_id: UserId },

    #[error("global theme {theme_id:?} cannot be deleted")]
    ThemeNotDeletable { theme_id: ThemeId },

    #[error("theme {theme_id:?} is currently selected by user {user_id:?}")]
    ThemeCurrentlySelected { theme_id: ThemeId, user_id: UserId },
}

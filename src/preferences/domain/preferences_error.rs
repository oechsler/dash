use thiserror::Error;

use super::TimeZoneError;
use crate::{shared::domain::UserId, theme::domain::ThemeId};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PreferencesError {
    #[error("preference timezone is invalid: {0}")]
    TimeZone(#[from] TimeZoneError),

    #[error("theme {theme_id:?} is currently selected by user {user_id:?}")]
    ThemeCurrentlySelected { theme_id: ThemeId, user_id: UserId },
}

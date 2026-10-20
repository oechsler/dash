use thiserror::Error;

use crate::domain::{
    catalog::ApplicationError,
    dashboard::{DashboardError, WebAddressError},
    icon::IconReferenceError,
    preferences::{PreferencesError, TimeZoneError},
    session::SessionError,
    theme::{ColorError, ThemeError},
    user::{IdentifierError, ProfilePictureUrlError, UserError},
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("dashboard domain error: {0}")]
    Dashboard(#[from] DashboardError),

    #[error("application catalog domain error: {0}")]
    Catalog(#[from] ApplicationError),

    #[error("user preferences domain error: {0}")]
    Preferences(#[from] PreferencesError),

    #[error("timezone domain error: {0}")]
    TimeZone(#[from] TimeZoneError),

    #[error("theme domain error: {0}")]
    Theme(#[from] ThemeError),

    #[error("theme color domain error: {0}")]
    Color(#[from] ColorError),

    #[error("session domain error: {0}")]
    Session(#[from] SessionError),

    #[error("user domain error: {0}")]
    User(#[from] UserError),

    #[error("user identifier domain error: {0}")]
    UserIdentifier(#[from] IdentifierError),

    #[error("profile picture domain error: {0}")]
    ProfilePicture(#[from] ProfilePictureUrlError),

    #[error("icon reference domain error: {0}")]
    IconReference(#[from] IconReferenceError),

    #[error("web address domain error: {0}")]
    WebAddress(#[from] WebAddressError),
}

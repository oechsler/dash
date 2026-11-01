use thiserror::Error;

use crate::{
    catalog::domain::CatalogError,
    dashboard::domain::{BookmarkError, CategoryError, DashboardError},
    icon::domain::IconReferenceError,
    preferences::domain::{PreferencesError, TimeZoneError},
    session::domain::SessionError,
    shared::domain::{IdentityError, WebAddressError},
    theme::domain::{ColorError, GlobalThemeIdError, ThemeError, ThemeIdError},
    user::domain::{ProfilePictureUrlError, UserError},
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationDomainError {
    #[error("shared identity error: {0}")]
    Identity(#[from] IdentityError),

    #[error("shared web address error: {0}")]
    WebAddress(#[from] WebAddressError),

    #[error("dashboard domain error: {0}")]
    Dashboard(#[from] DashboardError),

    #[error("application catalog domain error: {0}")]
    Catalog(#[from] CatalogError),

    #[error("icon reference domain error: {0}")]
    IconReference(#[from] IconReferenceError),

    #[error("user preferences domain error: {0}")]
    Preferences(#[from] PreferencesError),

    #[error("session domain error: {0}")]
    Session(#[from] SessionError),

    #[error("theme domain error: {0}")]
    Theme(#[from] ThemeError),

    #[error("user domain error: {0}")]
    User(#[from] UserError),
}

impl From<BookmarkError> for ApplicationDomainError {
    fn from(error: BookmarkError) -> Self {
        Self::Dashboard(error.into())
    }
}

impl From<CategoryError> for ApplicationDomainError {
    fn from(error: CategoryError) -> Self {
        Self::Dashboard(error.into())
    }
}

impl From<TimeZoneError> for ApplicationDomainError {
    fn from(error: TimeZoneError) -> Self {
        Self::Preferences(error.into())
    }
}

impl From<ColorError> for ApplicationDomainError {
    fn from(error: ColorError) -> Self {
        Self::Theme(error.into())
    }
}

impl From<ProfilePictureUrlError> for ApplicationDomainError {
    fn from(error: ProfilePictureUrlError) -> Self {
        Self::User(error.into())
    }
}

impl From<GlobalThemeIdError> for ApplicationDomainError {
    fn from(error: GlobalThemeIdError) -> Self {
        Self::Theme(error.into())
    }
}

impl From<ThemeIdError> for ApplicationDomainError {
    fn from(error: ThemeIdError) -> Self {
        Self::Theme(error.into())
    }
}

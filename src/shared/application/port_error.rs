use std::error::Error as StdError;

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

use super::ApplicationDomainError;

/// Error contract used by application-owned ports.
#[derive(Debug, Error)]
pub enum ApplicationPortError {
    #[error(transparent)]
    Domain(#[from] ApplicationDomainError),

    #[error("{0} was not found")]
    NotFound(&'static str),

    #[error("port operation `{operation}` failed: {source}")]
    Adapter {
        operation: &'static str,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
}

/// Result returned by application-owned ports and infrastructure adapters.
pub type PortResult<T> = Result<T, ApplicationPortError>;

impl ApplicationPortError {
    pub fn adapter<E>(operation: &'static str, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Adapter {
            operation,
            source: Box::new(source),
        }
    }
}

impl From<CatalogError> for ApplicationPortError {
    fn from(error: CatalogError) -> Self {
        Self::Domain(ApplicationDomainError::Catalog(error))
    }
}

impl From<DashboardError> for ApplicationPortError {
    fn from(error: DashboardError) -> Self {
        Self::Domain(ApplicationDomainError::Dashboard(error))
    }
}

impl From<WebAddressError> for ApplicationPortError {
    fn from(error: WebAddressError) -> Self {
        Self::Domain(ApplicationDomainError::WebAddress(error))
    }
}

impl From<IdentityError> for ApplicationPortError {
    fn from(error: IdentityError) -> Self {
        Self::Domain(ApplicationDomainError::Identity(error))
    }
}

impl From<PreferencesError> for ApplicationPortError {
    fn from(error: PreferencesError) -> Self {
        Self::Domain(ApplicationDomainError::Preferences(error))
    }
}

impl From<SessionError> for ApplicationPortError {
    fn from(error: SessionError) -> Self {
        Self::Domain(ApplicationDomainError::Session(error))
    }
}

impl From<ThemeError> for ApplicationPortError {
    fn from(error: ThemeError) -> Self {
        Self::Domain(ApplicationDomainError::Theme(error))
    }
}

impl From<UserError> for ApplicationPortError {
    fn from(error: UserError) -> Self {
        Self::Domain(ApplicationDomainError::User(error))
    }
}

impl From<BookmarkError> for ApplicationPortError {
    fn from(error: BookmarkError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<CategoryError> for ApplicationPortError {
    fn from(error: CategoryError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<IconReferenceError> for ApplicationPortError {
    fn from(error: IconReferenceError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<GlobalThemeIdError> for ApplicationPortError {
    fn from(error: GlobalThemeIdError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ThemeIdError> for ApplicationPortError {
    fn from(error: ThemeIdError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<TimeZoneError> for ApplicationPortError {
    fn from(error: TimeZoneError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ColorError> for ApplicationPortError {
    fn from(error: ColorError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ProfilePictureUrlError> for ApplicationPortError {
    fn from(error: ProfilePictureUrlError) -> Self {
        Self::Domain(error.into())
    }
}

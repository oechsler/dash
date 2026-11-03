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
    transfer::application::TransferError,
    user::domain::{ProfilePictureUrlError, UserError},
};

use super::ApplicationDomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] ApplicationDomainError),

    #[error("infrastructure operation `{operation}` failed: {source}")]
    Infrastructure {
        operation: &'static str,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },

    #[error("transfer application error: {0}")]
    Transfer(#[from] TransferError),

    #[error("{0} was not found")]
    NotFound(&'static str),

    #[error("the authenticated principal is not allowed to perform this operation")]
    Forbidden,

    #[error("the request is invalid")]
    BadRequest,
}

/// Result returned by application use cases and format adapters.
pub type ApplicationResult<T> = Result<T, ApplicationError>;

impl From<serde_json::Error> for ApplicationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Transfer(TransferError::Serialization(error))
    }
}

impl ApplicationError {
    pub fn infrastructure<E>(operation: &'static str, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Infrastructure {
            operation,
            source: Box::new(source),
        }
    }
}

impl From<super::ApplicationPortError> for ApplicationError {
    fn from(error: super::ApplicationPortError) -> Self {
        match error {
            super::ApplicationPortError::Domain(error) => Self::Domain(error),
            super::ApplicationPortError::NotFound(resource) => Self::NotFound(resource),
            super::ApplicationPortError::Adapter { operation, source } => {
                Self::Infrastructure { operation, source }
            }
        }
    }
}

impl From<DashboardError> for ApplicationError {
    fn from(error: DashboardError) -> Self {
        Self::Domain(ApplicationDomainError::Dashboard(error))
    }
}

impl From<WebAddressError> for ApplicationError {
    fn from(error: WebAddressError) -> Self {
        Self::Domain(ApplicationDomainError::WebAddress(error))
    }
}

impl From<IdentityError> for ApplicationError {
    fn from(error: IdentityError) -> Self {
        Self::Domain(ApplicationDomainError::Identity(error))
    }
}

impl From<CatalogError> for ApplicationError {
    fn from(error: CatalogError) -> Self {
        Self::Domain(ApplicationDomainError::Catalog(error))
    }
}

impl From<UserError> for ApplicationError {
    fn from(error: UserError) -> Self {
        Self::Domain(ApplicationDomainError::User(error))
    }
}

impl From<ThemeError> for ApplicationError {
    fn from(error: ThemeError) -> Self {
        Self::Domain(ApplicationDomainError::Theme(error))
    }
}

impl From<PreferencesError> for ApplicationError {
    fn from(error: PreferencesError) -> Self {
        Self::Domain(ApplicationDomainError::Preferences(error))
    }
}

impl From<SessionError> for ApplicationError {
    fn from(error: SessionError) -> Self {
        Self::Domain(ApplicationDomainError::Session(error))
    }
}

impl From<BookmarkError> for ApplicationError {
    fn from(error: BookmarkError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<CategoryError> for ApplicationError {
    fn from(error: CategoryError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<IconReferenceError> for ApplicationError {
    fn from(error: IconReferenceError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<GlobalThemeIdError> for ApplicationError {
    fn from(error: GlobalThemeIdError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ThemeIdError> for ApplicationError {
    fn from(error: ThemeIdError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<TimeZoneError> for ApplicationError {
    fn from(error: TimeZoneError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ColorError> for ApplicationError {
    fn from(error: ColorError) -> Self {
        Self::Domain(error.into())
    }
}

impl From<ProfilePictureUrlError> for ApplicationError {
    fn from(error: ProfilePictureUrlError) -> Self {
        Self::Domain(error.into())
    }
}

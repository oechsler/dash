use std::error::Error as StdError;

use thiserror::Error;

use crate::{
    catalog::domain::CatalogError,
    dashboard::domain::DashboardError,
    icon::domain::IconError,
    preferences::domain::PreferencesError,
    session::domain::SessionError,
    shared::domain::{DomainError, IdentityError, WebAddressError},
    theme::domain::ThemeError,
    user::domain::UserError,
};

/// Error contract used by application-owned ports.
#[derive(Debug, Error)]
pub enum ApplicationPortError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("{0} was not found")]
    NotFound(&'static str),

    #[error("port operation `{operation}` failed: {source}")]
    Adapter {
        operation: &'static str,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
}

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
        Self::Domain(DomainError::Catalog(error))
    }
}

impl From<DashboardError> for ApplicationPortError {
    fn from(error: DashboardError) -> Self {
        Self::Domain(DomainError::Dashboard(error))
    }
}

impl From<WebAddressError> for ApplicationPortError {
    fn from(error: WebAddressError) -> Self {
        Self::Domain(DomainError::WebAddress(error))
    }
}

impl From<IdentityError> for ApplicationPortError {
    fn from(error: IdentityError) -> Self {
        Self::Domain(DomainError::Identity(error))
    }
}

impl From<IconError> for ApplicationPortError {
    fn from(error: IconError) -> Self {
        Self::Domain(DomainError::Icon(error))
    }
}

impl From<PreferencesError> for ApplicationPortError {
    fn from(error: PreferencesError) -> Self {
        Self::Domain(DomainError::Preferences(error))
    }
}

impl From<SessionError> for ApplicationPortError {
    fn from(error: SessionError) -> Self {
        Self::Domain(DomainError::Session(error))
    }
}

impl From<ThemeError> for ApplicationPortError {
    fn from(error: ThemeError) -> Self {
        Self::Domain(DomainError::Theme(error))
    }
}

impl From<UserError> for ApplicationPortError {
    fn from(error: UserError) -> Self {
        Self::Domain(DomainError::User(error))
    }
}

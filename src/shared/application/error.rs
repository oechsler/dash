use std::error::Error as StdError;
use thiserror::Error;

use crate::{
    catalog::domain::CatalogError, dashboard::domain::DashboardError, icon::domain::IconError,
    preferences::domain::PreferencesError, session::domain::SessionError,
    shared::domain::DomainError, theme::domain::ThemeError, transfer::application::TransferError,
    user::domain::UserError,
};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("repository operation `{operation}` failed: {source}")]
    Repository {
        operation: &'static str,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },

    #[error("transfer application error: {0}")]
    Transfer(#[from] TransferError),

    #[error("{0} was not found")]
    NotFound(&'static str),
}

impl From<serde_json::Error> for ApplicationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Transfer(TransferError::Serialization(error))
    }
}

impl ApplicationError {
    pub fn repository<E>(operation: &'static str, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Repository {
            operation,
            source: Box::new(source),
        }
    }
}

impl From<DashboardError> for ApplicationError {
    fn from(error: DashboardError) -> Self {
        Self::Domain(DomainError::Dashboard(error))
    }
}

impl From<IconError> for ApplicationError {
    fn from(error: IconError) -> Self {
        Self::Domain(DomainError::Icon(error))
    }
}

impl From<CatalogError> for ApplicationError {
    fn from(error: CatalogError) -> Self {
        Self::Domain(DomainError::Catalog(error))
    }
}

impl From<UserError> for ApplicationError {
    fn from(error: UserError) -> Self {
        Self::Domain(DomainError::User(error))
    }
}

impl From<ThemeError> for ApplicationError {
    fn from(error: ThemeError) -> Self {
        Self::Domain(DomainError::Theme(error))
    }
}

impl From<PreferencesError> for ApplicationError {
    fn from(error: PreferencesError) -> Self {
        Self::Domain(DomainError::Preferences(error))
    }
}

impl From<SessionError> for ApplicationError {
    fn from(error: SessionError) -> Self {
        Self::Domain(DomainError::Session(error))
    }
}

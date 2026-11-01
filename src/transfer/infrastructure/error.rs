use thiserror::Error;

use crate::shared::{application::ApplicationPortError, infrastructure::DatabaseError};

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("snapshot owner does not match {aggregate} owner")]
    OwnerMismatch { aggregate: &'static str },

    #[error("theme is not a deletable user theme owned by the snapshot owner")]
    InvalidThemeOwnership,
}

impl InfrastructureError {
    pub fn database<E>(operation: &'static str, source: E) -> ApplicationPortError
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationPortError::adapter(operation, Self::Database(DatabaseError::source(source)))
    }

    pub fn owner_mismatch(
        operation: &'static str,
        aggregate: &'static str,
    ) -> ApplicationPortError {
        ApplicationPortError::adapter(operation, Self::OwnerMismatch { aggregate })
    }

    pub fn invalid_theme_owner(operation: &'static str) -> ApplicationPortError {
        ApplicationPortError::adapter(operation, Self::InvalidThemeOwnership)
    }
}

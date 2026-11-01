use thiserror::Error;

use super::CatalogApplicationError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogError {
    #[error("application catalog operation failed: {0}")]
    Application(#[from] CatalogApplicationError),

    #[error("application ordering must contain every catalog app exactly once")]
    InvalidApplicationOrder,
}

/// Result for operations that coordinate application-catalog entities.
pub type DomainResult<T> = Result<T, CatalogError>;

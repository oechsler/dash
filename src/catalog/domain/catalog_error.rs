use thiserror::Error;

use super::ApplicationError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogError {
    #[error("application catalog operation failed: {0}")]
    Application(#[from] ApplicationError),

    #[error("application ordering must contain every catalog app exactly once")]
    InvalidApplicationOrder,
}

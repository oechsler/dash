use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogApplicationError {
    #[error("application name must not be empty (received {value:?})")]
    EmptyName { value: String },

    #[error("health-check URL must be an absolute HTTP or HTTPS URL (received {value:?})")]
    InvalidHealthCheckUrl { value: String },
}

/// Result for application-catalog entity operations.
pub type CatalogApplicationResult<T> = Result<T, CatalogApplicationError>;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("application name must not be empty (received {value:?})")]
    EmptyName { value: String },

    #[error("health-check URL must be an absolute HTTP or HTTPS URL (received {value:?})")]
    InvalidHealthCheckUrl { value: String },
}

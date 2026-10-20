use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("application name must not be empty (received {value:?})")]
    EmptyName { value: String },
}

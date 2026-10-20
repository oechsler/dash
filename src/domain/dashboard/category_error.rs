use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CategoryError {
    #[error("category name must not be empty (received {value:?})")]
    EmptyName { value: String },
}

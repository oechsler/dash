use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BookmarkError {
    #[error("bookmark name must not be empty (received {value:?})")]
    EmptyName { value: String },
}

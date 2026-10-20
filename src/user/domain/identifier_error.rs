use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IdentifierError {
    #[error("user ID must not be empty (received {value:?})")]
    EmptyUserId { value: String },

    #[error("group ID must not be empty (received {value:?})")]
    EmptyGroupId { value: String },
}

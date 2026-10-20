use thiserror::Error;

use super::IconReferenceError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IconError {
    #[error("icon reference is invalid: {0}")]
    InvalidReference(#[from] IconReferenceError),
}

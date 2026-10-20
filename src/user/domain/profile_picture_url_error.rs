use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProfilePictureUrlError {
    #[error("invalid profile picture URL {value:?}: {reason}")]
    InvalidUrl { value: String, reason: String },
}

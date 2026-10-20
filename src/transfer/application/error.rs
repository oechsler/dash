use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("user-data export signature is missing")]
    MissingSignature,

    #[error("user-data export signature is invalid")]
    InvalidSignature,

    #[error("unsupported user-data export version {0}")]
    UnsupportedVersion(u32),

    #[error("invalid user-data export: {0}")]
    InvalidDocument(String),

    #[error("transfer serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

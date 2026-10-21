use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferOperation {
    Import,
    Export,
}

impl std::fmt::Display for TransferOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Import => formatter.write_str("import"),
            Self::Export => formatter.write_str("export"),
        }
    }
}

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("user-data export signature is missing")]
    MissingSignature,

    #[error("user-data export signature is invalid")]
    InvalidSignature,

    #[error("unsupported transfer format version {0}")]
    UnsupportedVersion(u32),

    #[error("transfer format version {version} does not support {operation}")]
    UnsupportedOperation {
        version: u32,
        operation: TransferOperation,
    },

    #[error("invalid user-data export: {0}")]
    InvalidDocument(String),

    #[error("transfer serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

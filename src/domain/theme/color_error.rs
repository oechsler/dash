use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ColorError {
    #[error("color must be a 3- or 6-digit hex color (received {value:?})")]
    InvalidHex { value: String },
}

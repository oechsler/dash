use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TimeZoneError {
    #[error("timezone must be `auto` or a valid IANA timezone (received {value:?})")]
    InvalidIanaName { value: String },
}

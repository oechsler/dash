use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WebAddressError {
    #[error("web address is malformed: {value:?} ({reason})")]
    Malformed { value: String, reason: String },

    #[error("web address must include a host: {value:?}")]
    MissingHost { value: String },
}

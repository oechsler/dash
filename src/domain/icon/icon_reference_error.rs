use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IconReferenceError {
    #[error("icon reference must not be empty (received {value:?})")]
    Empty { value: String },
}

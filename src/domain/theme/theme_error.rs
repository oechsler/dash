use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThemeError {
    #[error("theme name must not be empty (received {value:?})")]
    EmptyName { value: String },

    #[error("global theme ID must not be empty")]
    EmptyGlobalId,
}

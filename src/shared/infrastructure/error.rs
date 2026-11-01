use std::error::Error as StdError;

use thiserror::Error;

use crate::shared::application::ApplicationPortError;

/// Error raised while reading or writing persisted data.
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database operation failed: {source}")]
    Source {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },

    #[error("value `{value}` for `{field}` is invalid")]
    InvalidValue { field: &'static str, value: String },

    #[error("expected persisted `{entity}` record was not found")]
    ExpectedRecordMissing { entity: &'static str },
}

impl DatabaseError {
    pub fn source<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::Source {
            source: Box::new(source),
        }
    }

    pub fn invalid_value(field: &'static str, value: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            value: value.into(),
        }
    }

    pub fn expected_record_missing(entity: &'static str) -> Self {
        Self::ExpectedRecordMissing { entity }
    }
}

/// Adds the application-port operation to a database error while preserving its source chain.
#[derive(Debug, Error)]
#[error("database adapter error: {source}")]
pub struct DatabaseAdapterError {
    #[source]
    source: DatabaseError,
}

impl DatabaseAdapterError {
    pub fn database<E>(operation: &'static str, source: E) -> ApplicationPortError
    where
        E: StdError + Send + Sync + 'static,
    {
        ApplicationPortError::adapter(operation, Self::from(DatabaseError::source(source)))
    }

    pub fn persistence(operation: &'static str, source: DatabaseError) -> ApplicationPortError {
        ApplicationPortError::adapter(operation, Self::from(source))
    }

    pub fn invalid_value(field: &'static str, value: impl Into<String>) -> Self {
        DatabaseError::invalid_value(field, value).into()
    }
}

impl From<DatabaseError> for DatabaseAdapterError {
    fn from(source: DatabaseError) -> Self {
        Self { source }
    }
}

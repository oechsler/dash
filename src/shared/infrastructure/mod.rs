mod error;
pub mod migrations;
#[cfg(feature = "postgres")]
mod postgres_connector;
pub(crate) mod schema;

pub use error::{DatabaseAdapterError, DatabaseError};

#[cfg(test)]
mod error_test;
#[cfg(test)]
mod schema_test;

#[cfg(feature = "postgres")]
pub use postgres_connector::PostgresConnector;

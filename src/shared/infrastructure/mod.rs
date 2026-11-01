mod error;
pub(crate) mod schema;

pub use error::{DatabaseAdapterError, DatabaseError};

#[cfg(test)]
mod error_test;
#[cfg(test)]
mod schema_test;

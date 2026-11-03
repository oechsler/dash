mod error;

pub use error::{DatabaseAdapterError, DatabaseError};

#[cfg(test)]
mod error_test;
#[cfg(test)]
mod schema_test;

//! Composition root for application-wide infrastructure.

pub mod migrations;
#[cfg(feature = "postgres")]
mod postgres_connector;

#[cfg(feature = "postgres")]
pub use postgres_connector::{PostgresConnector, PostgresConnectorResult};

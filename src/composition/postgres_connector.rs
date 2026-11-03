use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;

use super::migrations::Migrator;

/// Result returned while connecting to PostgreSQL or applying migrations.
pub type PostgresConnectorResult<T> = Result<T, sea_orm::DbErr>;

/// Owns the shared PostgreSQL connection pool used by the application.
#[derive(Clone, Debug)]
pub struct PostgresConnector {
    connection: Arc<DatabaseConnection>,
}

impl PostgresConnector {
    pub async fn connect(url: &str) -> PostgresConnectorResult<Self> {
        let mut options = ConnectOptions::new(url.to_owned());
        options.sqlx_logging(false);
        let connection = Database::connect(options).await?;
        Ok(Self {
            connection: Arc::new(connection),
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        self.connection.as_ref()
    }

    /// Cloned handle for constructing independent infrastructure adapters.
    pub fn connection_handle(&self) -> Arc<DatabaseConnection> {
        Arc::clone(&self.connection)
    }

    /// Applies versioned schema migrations to the configured database.
    pub async fn migrate(&self) -> PostgresConnectorResult<()> {
        Migrator::up(self.connection.as_ref(), None).await
    }
}

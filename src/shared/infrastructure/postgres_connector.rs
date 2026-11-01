use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;

use super::migrations::Migrator;

/// Owns the application's shared PostgreSQL connection pool.
#[derive(Clone, Debug)]
pub struct PostgresConnector {
    connection: Arc<DatabaseConnection>,
}

impl PostgresConnector {
    pub async fn connect(url: &str) -> Result<Self, DbErr> {
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

    /// Applies versioned schema migrations to the configured database.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        Migrator::up(self.connection.as_ref(), None).await
    }
}

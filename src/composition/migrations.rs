use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Collects context migrations at the composition boundary. The
/// actual schema lives next to each context's persistence implementation.
pub struct Migrator;

#[cfg(test)]
#[path = "migrations_test.rs"]
mod migrations_test;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(crate::user::infrastructure::sea_orm::UserSchemaV1),
            Box::new(crate::dashboard::infrastructure::sea_orm::DashboardSchemaV1),
            Box::new(crate::catalog::infrastructure::sea_orm::CatalogSchemaV1),
            Box::new(crate::theme::infrastructure::sea_orm::ThemeSchemaV1),
            Box::new(crate::preferences::infrastructure::sea_orm::PreferencesSchemaV1),
            Box::new(crate::session::infrastructure::sea_orm::SessionSchemaV1),
        ]
    }
}

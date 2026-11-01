use sea_orm_migration::prelude::*;

/// Collects the bounded-context migrations for the application runner. The
/// actual schema lives next to each context's persistence implementation.
pub struct Migrator;

#[async_trait::async_trait]
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

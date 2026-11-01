use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Collects context migrations at the composition boundary. The
/// actual schema lives next to each context's persistence implementation.
pub struct Migrator;

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

#[cfg(test)]
mod tests {
    use sea_orm_migration::MigratorTrait;

    use super::Migrator;

    #[test]
    fn derived_migration_names_preserve_the_persisted_ids_and_order() {
        let names: Vec<_> = Migrator::migrations()
            .into_iter()
            .map(|migration| migration.name().to_owned())
            .collect();

        assert_eq!(
            names,
            [
                "m0001_user_schema_v1",
                "m0002_dashboard_schema_v1",
                "m0003_catalog_schema_v1",
                "m0004_theme_schema_v1",
                "m0005_preferences_schema_v1",
                "m0006_session_schema_v1",
            ]
        );
    }
}

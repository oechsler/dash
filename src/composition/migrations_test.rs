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

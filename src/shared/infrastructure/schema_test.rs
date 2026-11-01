use sea_orm::{DatabaseBackend, EntityTrait, Schema};

fn assert_declared_foreign_key<E: EntityTrait>(entity: E) {
    let schema = Schema::new(DatabaseBackend::Postgres);
    let statement = schema.create_table_from_entity(entity);
    let sql = DatabaseBackend::Postgres.build(&statement).to_string();
    assert!(
        sql.contains("FOREIGN KEY"),
        "{} lacks its declared relation: {sql}",
        entity.table_name()
    );
    assert!(
        sql.contains("ON DELETE CASCADE"),
        "{} lacks its delete policy: {sql}",
        entity.table_name()
    );
}

#[test]
fn entity_schema_contains_declared_foreign_keys() {
    assert_declared_foreign_key(crate::user::infrastructure::sea_orm::entities::identity::Entity);
    assert_declared_foreign_key(
        crate::dashboard::infrastructure::sea_orm::entities::dashboard::Entity,
    );
    assert_declared_foreign_key(
        crate::dashboard::infrastructure::sea_orm::entities::category::Entity,
    );
    assert_declared_foreign_key(
        crate::dashboard::infrastructure::sea_orm::entities::bookmark::Entity,
    );
    assert_declared_foreign_key(
        crate::catalog::infrastructure::sea_orm::entities::visibility_group::Entity,
    );
    assert_declared_foreign_key(crate::theme::infrastructure::sea_orm::entities::Entity);
    assert_declared_foreign_key(
        crate::preferences::infrastructure::sea_orm::entities::preferences::Entity,
    );
    assert_declared_foreign_key(
        crate::preferences::infrastructure::sea_orm::entities::recent_application::Entity,
    );
    assert_declared_foreign_key(crate::session::infrastructure::sea_orm::entities::session::Entity);
}

use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct CatalogSchemaV1;

#[async_trait]
impl MigrationTrait for CatalogSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        execute_postgres_ddl(
            manager,
            &[
                r#"
                    CREATE TABLE "catalog_applications" (
                        "id" uuid NOT NULL PRIMARY KEY,
                        "name" varchar NOT NULL,
                        "icon" varchar NOT NULL,
                        "address" varchar NOT NULL,
                        "health_check_url" varchar,
                        "health_check_status_class" varchar,
                        "health_check_ignore_tls_errors" bool NOT NULL,
                        "health_status" varchar
                    )
                "#,
                r#"
                    CREATE TABLE "application_visibility_groups" (
                        "application_id" uuid NOT NULL,
                        "group_id" varchar NOT NULL,
                        CONSTRAINT "pk-application_visibility_groups"
                            PRIMARY KEY ("application_id", "group_id"),
                        CONSTRAINT "fk-application_visibility_groups-application_id"
                            FOREIGN KEY ("application_id")
                            REFERENCES "catalog_applications" ("id") ON DELETE CASCADE
                    )
                "#,
                r#"
                    CREATE TABLE "application_catalog_settings" (
                        "singleton" bool NOT NULL PRIMARY KEY,
                        "sort_order" varchar NOT NULL,
                        "application_order" varchar NOT NULL
                    )
                "#,
            ],
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        for table in [
            "application_catalog_settings",
            "application_visibility_groups",
            "catalog_applications",
        ] {
            manager
                .drop_table(Table::drop().table(Alias::new(table)).to_owned())
                .await?;
        }
        Ok(())
    }
}

use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct DashboardSchemaV1;

#[async_trait]
impl MigrationTrait for DashboardSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        execute_postgres_ddl(
            manager,
            &[
                r#"
                    CREATE TABLE "dashboards" (
                        "owner_id" varchar NOT NULL PRIMARY KEY,
                        "categories_sort_order" varchar NOT NULL,
                        CONSTRAINT "fk-dashboards-owner_id"
                            FOREIGN KEY ("owner_id") REFERENCES "users" ("id") ON DELETE CASCADE
                    )
                "#,
                r#"
                    CREATE TABLE "dashboard_categories" (
                        "id" uuid NOT NULL PRIMARY KEY,
                        "owner_id" varchar NOT NULL,
                        "name" varchar NOT NULL,
                        "is_shelved" bool NOT NULL,
                        "sort_order" varchar NOT NULL,
                        "position" integer NOT NULL,
                        CONSTRAINT "fk-dashboard_categories-owner_id"
                            FOREIGN KEY ("owner_id")
                            REFERENCES "dashboards" ("owner_id") ON DELETE CASCADE
                    )
                "#,
                r#"
                    CREATE TABLE "dashboard_bookmarks" (
                        "id" uuid NOT NULL PRIMARY KEY,
                        "category_id" uuid NOT NULL,
                        "name" varchar NOT NULL,
                        "icon" varchar NOT NULL,
                        "address" varchar NOT NULL,
                        "last_accessed_at" timestamp with time zone,
                        "position" integer NOT NULL,
                        CONSTRAINT "fk-dashboard_bookmarks-category_id"
                            FOREIGN KEY ("category_id")
                            REFERENCES "dashboard_categories" ("id") ON DELETE CASCADE
                    )
                "#,
            ],
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        for table in ["dashboard_bookmarks", "dashboard_categories", "dashboards"] {
            manager
                .drop_table(Table::drop().table(Alias::new(table)).to_owned())
                .await?;
        }
        Ok(())
    }
}

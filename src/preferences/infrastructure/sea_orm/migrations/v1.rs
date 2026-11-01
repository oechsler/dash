use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

pub struct PreferencesSchemaV1;
impl MigrationName for PreferencesSchemaV1 {
    fn name(&self) -> &str {
        "m0005_preferences_schema_v1"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for PreferencesSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        execute_postgres_ddl(
            manager,
            &[
                r#"CREATE TABLE "user_preferences" ( "owner_id" varchar NOT NULL PRIMARY KEY, "light_theme_id" varchar, "dark_theme_id" varchar, "language" varchar NOT NULL, "time_zone" varchar NOT NULL, CONSTRAINT "fk-user_preferences-owner_id" FOREIGN KEY ("owner_id") REFERENCES "users" ("id") ON DELETE CASCADE )"#,
                r#"CREATE TABLE "user_recent_applications" ( "owner_id" varchar NOT NULL, "application_id" uuid NOT NULL, "position" integer NOT NULL, CONSTRAINT "pk-user_recent_applications" PRIMARY KEY ("owner_id", "application_id"), CONSTRAINT "fk-user_recent_applications-owner_id" FOREIGN KEY ("owner_id") REFERENCES "users" ("id") ON DELETE CASCADE, CONSTRAINT "fk-user_recent_applications-application_id" FOREIGN KEY ("application_id") REFERENCES "catalog_applications" ("id") ON DELETE CASCADE )"#,
            ],
        )
        .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in ["user_recent_applications", "user_preferences"] {
            manager
                .drop_table(Table::drop().table(Alias::new(table)).to_owned())
                .await?;
        }
        Ok(())
    }
}

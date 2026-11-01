use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

pub struct ThemeSchemaV1;
impl MigrationName for ThemeSchemaV1 {
    fn name(&self) -> &str {
        "m0004_theme_schema_v1"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for ThemeSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        execute_postgres_ddl(
            manager,
            &[r#"CREATE TABLE "themes" ( "id" varchar NOT NULL PRIMARY KEY, "owner_id" varchar, "name" varchar NOT NULL, "mode" varchar NOT NULL, "primary_color" varchar NOT NULL, "secondary_color" varchar NOT NULL, "tertiary_color" varchar NOT NULL, CONSTRAINT "fk-themes-owner_id" FOREIGN KEY ("owner_id") REFERENCES "users" ("id") ON DELETE CASCADE )"#],
        )
        .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("themes")).to_owned())
            .await
    }
}

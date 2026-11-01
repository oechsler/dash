use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

pub struct UserSchemaV1;
impl MigrationName for UserSchemaV1 {
    fn name(&self) -> &str {
        "m0001_user_schema_v1"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for UserSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        execute_postgres_ddl(
            manager,
            &[
                r#"CREATE TABLE "users" ( "id" varchar NOT NULL PRIMARY KEY, "username" varchar NOT NULL, "first_name" varchar, "last_name" varchar, "profile_picture" varchar )"#,
                r#"CREATE TABLE "external_identities" ( "issuer" varchar NOT NULL, "subject" varchar NOT NULL, "user_id" varchar NOT NULL, CONSTRAINT "pk-external_identities" PRIMARY KEY ("issuer", "subject"), CONSTRAINT "fk-external_identities-user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE )"#,
            ],
        )
        .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in ["external_identities", "users"] {
            manager
                .drop_table(Table::drop().table(Alias::new(table)).to_owned())
                .await?;
        }
        Ok(())
    }
}

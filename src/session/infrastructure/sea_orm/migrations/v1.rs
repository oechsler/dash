use sea_orm_migration::prelude::*;

use crate::shared::infrastructure::schema::execute_postgres_ddl;

pub struct SessionSchemaV1;
impl MigrationName for SessionSchemaV1 {
    fn name(&self) -> &str {
        "m0006_session_schema_v1"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for SessionSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        execute_postgres_ddl(
            manager,
            &[r#"CREATE TABLE "sessions" ( "id" uuid NOT NULL PRIMARY KEY, "owner_id" varchar NOT NULL, "long_lived" bool NOT NULL, "browser" varchar, "operating_system" varchar, "ip_address" varchar, "issued_at" timestamp with time zone NOT NULL, "last_accessed_at" timestamp with time zone NOT NULL, "expires_at" timestamp with time zone NOT NULL, "terminated_at" timestamp with time zone, CONSTRAINT "fk-sessions-owner_id" FOREIGN KEY ("owner_id") REFERENCES "users" ("id") ON DELETE CASCADE )"#],
        )
        .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("sessions")).to_owned())
            .await
    }
}

use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct UserSchemaV1;

#[async_trait]
impl MigrationTrait for UserSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(ColumnDef::new(Users::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::FirstName).string())
                    .col(ColumnDef::new(Users::LastName).string())
                    .col(ColumnDef::new(Users::ProfilePicture).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ExternalIdentities::Table)
                    .col(
                        ColumnDef::new(ExternalIdentities::Issuer)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalIdentities::Subject)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExternalIdentities::UserId)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-external_identities")
                            .col(ExternalIdentities::Issuer)
                            .col(ExternalIdentities::Subject),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-external_identities-user_id")
                            .from(ExternalIdentities::Table, ExternalIdentities::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        for table in ["external_identities", "users"] {
            manager
                .drop_table(Table::drop().table(Alias::new(table)).to_owned())
                .await?;
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    FirstName,
    LastName,
    ProfilePicture,
}

#[derive(DeriveIden)]
enum ExternalIdentities {
    Table,
    Issuer,
    Subject,
    UserId,
}

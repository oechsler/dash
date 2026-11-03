use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct SessionSchemaV1;

#[async_trait]
impl MigrationTrait for SessionSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .col(ColumnDef::new(Sessions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Sessions::OwnerId).string().not_null())
                    .col(ColumnDef::new(Sessions::LongLived).boolean().not_null())
                    .col(ColumnDef::new(Sessions::Browser).string())
                    .col(ColumnDef::new(Sessions::OperatingSystem).string())
                    .col(ColumnDef::new(Sessions::IpAddress).string())
                    .col(
                        ColumnDef::new(Sessions::IssuedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Sessions::LastAccessedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Sessions::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Sessions::TerminatedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sessions-owner_id")
                            .from(Sessions::Table, Sessions::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .drop_table(Table::drop().table(Alias::new("sessions")).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    OwnerId,
    LongLived,
    Browser,
    OperatingSystem,
    IpAddress,
    IssuedAt,
    LastAccessedAt,
    ExpiresAt,
    TerminatedAt,
}

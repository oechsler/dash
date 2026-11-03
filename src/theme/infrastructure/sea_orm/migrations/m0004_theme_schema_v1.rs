use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct ThemeSchemaV1;

#[async_trait]
impl MigrationTrait for ThemeSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(Themes::Table)
                    .col(ColumnDef::new(Themes::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Themes::OwnerId).string())
                    .col(ColumnDef::new(Themes::Name).string().not_null())
                    .col(ColumnDef::new(Themes::Mode).string().not_null())
                    .col(ColumnDef::new(Themes::PrimaryColor).string().not_null())
                    .col(ColumnDef::new(Themes::SecondaryColor).string().not_null())
                    .col(ColumnDef::new(Themes::TertiaryColor).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-themes-owner_id")
                            .from(Themes::Table, Themes::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .drop_table(Table::drop().table(Alias::new("themes")).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Themes {
    Table,
    Id,
    OwnerId,
    Name,
    Mode,
    PrimaryColor,
    SecondaryColor,
    TertiaryColor,
}

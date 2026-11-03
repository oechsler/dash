use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct PreferencesSchemaV1;

#[async_trait]
impl MigrationTrait for PreferencesSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(UserPreferences::Table)
                    .col(
                        ColumnDef::new(UserPreferences::OwnerId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserPreferences::LightThemeId).string())
                    .col(ColumnDef::new(UserPreferences::DarkThemeId).string())
                    .col(
                        ColumnDef::new(UserPreferences::Language)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPreferences::TimeZone)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_preferences-owner_id")
                            .from(UserPreferences::Table, UserPreferences::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserRecentApplications::Table)
                    .col(
                        ColumnDef::new(UserRecentApplications::OwnerId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRecentApplications::ApplicationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRecentApplications::Position)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-user_recent_applications")
                            .col(UserRecentApplications::OwnerId)
                            .col(UserRecentApplications::ApplicationId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_recent_applications-owner_id")
                            .from(
                                UserRecentApplications::Table,
                                UserRecentApplications::OwnerId,
                            )
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_recent_applications-application_id")
                            .from(
                                UserRecentApplications::Table,
                                UserRecentApplications::ApplicationId,
                            )
                            .to(CatalogApplications::Table, CatalogApplications::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> MigrationResult<()> {
        for table in ["user_recent_applications", "user_preferences"] {
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
}
#[derive(DeriveIden)]
enum CatalogApplications {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum UserPreferences {
    Table,
    OwnerId,
    LightThemeId,
    DarkThemeId,
    Language,
    TimeZone,
}
#[derive(DeriveIden)]
enum UserRecentApplications {
    Table,
    OwnerId,
    ApplicationId,
    Position,
}

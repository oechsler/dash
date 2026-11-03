use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct CatalogSchemaV1;

#[async_trait]
impl MigrationTrait for CatalogSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogApplications::Table)
                    .col(
                        ColumnDef::new(CatalogApplications::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CatalogApplications::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CatalogApplications::Icon)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CatalogApplications::Address)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CatalogApplications::HealthCheckUrl).string())
                    .col(ColumnDef::new(CatalogApplications::HealthCheckStatusClass).string())
                    .col(
                        ColumnDef::new(CatalogApplications::HealthCheckIgnoreTlsErrors)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CatalogApplications::HealthStatus).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ApplicationVisibilityGroups::Table)
                    .col(
                        ColumnDef::new(ApplicationVisibilityGroups::ApplicationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ApplicationVisibilityGroups::GroupId)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-application_visibility_groups")
                            .col(ApplicationVisibilityGroups::ApplicationId)
                            .col(ApplicationVisibilityGroups::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-application_visibility_groups-application_id")
                            .from(
                                ApplicationVisibilityGroups::Table,
                                ApplicationVisibilityGroups::ApplicationId,
                            )
                            .to(CatalogApplications::Table, CatalogApplications::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ApplicationCatalogSettings::Table)
                    .col(
                        ColumnDef::new(ApplicationCatalogSettings::Singleton)
                            .boolean()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ApplicationCatalogSettings::SortOrder)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ApplicationCatalogSettings::ApplicationOrder)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
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

#[derive(DeriveIden)]
enum CatalogApplications {
    Table,
    Id,
    Name,
    Icon,
    Address,
    HealthCheckUrl,
    HealthCheckStatusClass,
    HealthCheckIgnoreTlsErrors,
    HealthStatus,
}
#[derive(DeriveIden)]
enum ApplicationVisibilityGroups {
    Table,
    ApplicationId,
    GroupId,
}
#[derive(DeriveIden)]
enum ApplicationCatalogSettings {
    Table,
    Singleton,
    SortOrder,
    ApplicationOrder,
}

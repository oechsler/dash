use async_trait::async_trait;
use sea_orm_migration::prelude::*;

/// Result returned by this migration's SeaORM hooks.
pub type MigrationResult<T> = Result<T, DbErr>;

#[derive(DeriveMigrationName)]
pub struct DashboardSchemaV1;

#[async_trait]
impl MigrationTrait for DashboardSchemaV1 {
    async fn up(&self, manager: &SchemaManager) -> MigrationResult<()> {
        manager
            .create_table(
                Table::create()
                    .table(Dashboards::Table)
                    .col(
                        ColumnDef::new(Dashboards::OwnerId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Dashboards::CategoriesSortOrder)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-dashboards-owner_id")
                            .from(Dashboards::Table, Dashboards::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DashboardCategories::Table)
                    .col(
                        ColumnDef::new(DashboardCategories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DashboardCategories::OwnerId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DashboardCategories::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DashboardCategories::IsShelved)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DashboardCategories::SortOrder)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DashboardCategories::Position)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-dashboard_categories-owner_id")
                            .from(DashboardCategories::Table, DashboardCategories::OwnerId)
                            .to(Dashboards::Table, Dashboards::OwnerId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DashboardBookmarks::Table)
                    .col(
                        ColumnDef::new(DashboardBookmarks::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DashboardBookmarks::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DashboardBookmarks::Name).string().not_null())
                    .col(ColumnDef::new(DashboardBookmarks::Icon).string().not_null())
                    .col(
                        ColumnDef::new(DashboardBookmarks::Address)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DashboardBookmarks::LastAccessedAt)
                            .timestamp_with_time_zone(),
                    )
                    .col(
                        ColumnDef::new(DashboardBookmarks::Position)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-dashboard_bookmarks-category_id")
                            .from(DashboardBookmarks::Table, DashboardBookmarks::CategoryId)
                            .to(DashboardCategories::Table, DashboardCategories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
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

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Dashboards {
    Table,
    OwnerId,
    CategoriesSortOrder,
}
#[derive(DeriveIden)]
enum DashboardCategories {
    Table,
    Id,
    OwnerId,
    Name,
    IsShelved,
    SortOrder,
    Position,
}
#[derive(DeriveIden)]
enum DashboardBookmarks {
    Table,
    Id,
    CategoryId,
    Name,
    Icon,
    Address,
    LastAccessedAt,
    Position,
}

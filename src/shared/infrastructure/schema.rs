use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

/// Executes frozen PostgreSQL DDL statements for a versioned migration.
/// Future schema changes belong in a new migration, never in an old snapshot.
pub async fn execute_postgres_ddl(
    manager: &SchemaManager<'_>,
    statements: &[&str],
) -> Result<(), DbErr> {
    let connection = manager.get_connection();
    for statement in statements {
        connection.execute_unprepared(statement).await?;
    }
    Ok(())
}

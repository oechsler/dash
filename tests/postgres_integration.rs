#![cfg(feature = "postgres")]

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement, TransactionTrait};
use uuid::Uuid;

use dash::shared::infrastructure::PostgresConnector;

/// Runs against a disposable PostgreSQL database; enabled explicitly in CI.
#[tokio::test]
#[ignore = "requires DASH_TEST_DATABASE_URL pointing to a disposable PostgreSQL database"]
async fn migrations_are_repeatable_and_user_deletion_cascades() {
    let database_url = std::env::var("DASH_TEST_DATABASE_URL")
        .expect("DASH_TEST_DATABASE_URL must point to a disposable test database");
    let connector = PostgresConnector::connect(&database_url)
        .await
        .expect("connect to PostgreSQL");
    connector.migrate().await.expect("apply migrations");
    connector
        .migrate()
        .await
        .expect("reapplying migrations is safe");

    let user_id = Uuid::new_v4().to_string();
    let connection = connector.connection();
    let missing_user_id = Uuid::new_v4().to_string();
    let rejected_identity = connection
        .execute_unprepared(&format!(
            "INSERT INTO external_identities (issuer, subject, user_id) \
             VALUES ('integration-test', '{missing_user_id}', '{missing_user_id}')"
        ))
        .await;
    assert!(
        rejected_identity.is_err(),
        "the user foreign key is enforced"
    );

    let rolled_back_user_id = Uuid::new_v4().to_string();
    let transaction = connection.begin().await.expect("begin transaction");
    transaction
        .execute_unprepared(&format!(
            "INSERT INTO users (id, username) VALUES ('{rolled_back_user_id}', 'rolled-back')"
        ))
        .await
        .expect("insert user in transaction");
    transaction.rollback().await.expect("roll back transaction");
    let rolled_back_count = connection
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!("SELECT COUNT(*) AS count FROM users WHERE id = '{rolled_back_user_id}'"),
        ))
        .await
        .expect("query rolled-back user")
        .expect("count query returns a row")
        .try_get::<i64>("", "count")
        .expect("count is an integer");
    assert_eq!(rolled_back_count, 0, "rollback removes the inserted user");

    connection
        .execute_unprepared(&format!(
            "INSERT INTO users (id, username) VALUES ('{user_id}', 'integration-test')"
        ))
        .await
        .expect("insert test user");
    connection
        .execute_unprepared(&format!(
            "INSERT INTO external_identities (issuer, subject, user_id) \
             VALUES ('integration-test', '{user_id}', '{user_id}')"
        ))
        .await
        .expect("insert test identity");

    connection
        .execute_unprepared(&format!("DELETE FROM users WHERE id = '{user_id}'"))
        .await
        .expect("delete test user");
    let remaining_identity = connection
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!(
                "SELECT COUNT(*) AS count FROM external_identities \
                 WHERE issuer = 'integration-test' AND subject = '{user_id}'"
            ),
        ))
        .await
        .expect("query identity count")
        .expect("count query returns a row")
        .try_get::<i64>("", "count")
        .expect("count is an integer");

    assert_eq!(remaining_identity, 0, "user deletion should cascade");
}

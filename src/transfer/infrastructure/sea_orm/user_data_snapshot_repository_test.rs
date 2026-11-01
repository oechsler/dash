use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::sync::Arc;

use crate::{
    catalog::domain::{Application, ApplicationCatalogSettings, ApplicationVisibility},
    icon::domain::IconReference,
    shared::domain::UserId,
    shared::domain::WebAddress,
    transfer::{
        application::{models::UserDataSnapshot, ports::UserDataSnapshotWriter},
        infrastructure::sea_orm::UserDataSnapshotRepository,
    },
};

#[tokio::test]
async fn snapshot_catalog_writes_share_one_transaction() {
    let connection = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![
                MockExecResult::default(),
                MockExecResult::default(),
                MockExecResult::default(),
            ])
            .into_connection(),
    );
    let repository = UserDataSnapshotRepository::new(connection.clone());
    let owner = UserId::try_from("owner-1").unwrap();
    let application = Application::builder()
        .name("Portal")
        .icon(IconReference::try_from("pack:portal").unwrap())
        .address(WebAddress::try_from("https://portal.example").unwrap())
        .visibility(ApplicationVisibility::everyone())
        .build()
        .unwrap();
    let snapshot = UserDataSnapshot {
        applications: vec![application],
        application_catalog_settings: Some(ApplicationCatalogSettings::default()),
        ..UserDataSnapshot::default()
    };

    repository.save(&owner, &snapshot).await.unwrap();

    drop(repository);
    let connection = Arc::try_unwrap(connection).unwrap();
    let transaction_log = connection.into_transaction_log();
    assert_eq!(transaction_log.len(), 1);
    assert_eq!(transaction_log[0].statements().len(), 5);
}

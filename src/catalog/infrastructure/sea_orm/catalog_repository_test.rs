use sea_orm::{DatabaseBackend, MockDatabase};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    catalog::application::ports::ApplicationReader,
    catalog::infrastructure::sea_orm::{
        CatalogRepository,
        entities::{application, visibility_group},
    },
};

#[tokio::test]
async fn list_loads_visibility_groups_in_a_single_batch() {
    let first_id = Uuid::new_v4();
    let second_id = Uuid::new_v4();
    let connection = Arc::new(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[
                application::Model {
                    id: first_id,
                    name: "First".to_owned(),
                    icon: "pack:first".to_owned(),
                    address: "https://first.example".to_owned(),
                    health_check_url: None,
                    health_check_status_class: None,
                    health_check_ignore_tls_errors: false,
                    health_status: None,
                },
                application::Model {
                    id: second_id,
                    name: "Second".to_owned(),
                    icon: "pack:second".to_owned(),
                    address: "https://second.example".to_owned(),
                    health_check_url: None,
                    health_check_status_class: None,
                    health_check_ignore_tls_errors: false,
                    health_status: None,
                },
            ]])
            .append_query_results([[visibility_group::Model {
                application_id: first_id,
                group_id: "engineering".to_owned(),
            }]])
            .into_connection(),
    );
    let repository = CatalogRepository::new(connection.clone());

    let applications = repository.list().await.unwrap();

    assert_eq!(applications.len(), 2);
    assert_eq!(applications[0].id().as_uuid(), first_id);
    assert_eq!(applications[1].id().as_uuid(), second_id);
    let first_groups = applications[0]
        .visibility()
        .visible_to_groups()
        .expect("first application should be group-restricted");
    assert!(
        first_groups
            .iter()
            .any(|group| group.as_str() == "engineering")
    );
    assert!(applications[1].visibility().visible_to_groups().is_none());

    drop(repository);
    let connection = Arc::try_unwrap(connection).unwrap();
    assert_eq!(connection.into_transaction_log().len(), 2);
}

use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    transfer::{
        application::queries::{ExportUserData, ExportUserDataInput},
        application::{models::UserDataSnapshot, ports::UserDataSnapshotReader},
    },
    user::domain::UserId,
};

struct EmptyUserData;

#[async_trait]
impl UserDataSnapshotReader for EmptyUserData {
    async fn load(&self, _owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        Ok(UserDataSnapshot::default())
    }
}

#[tokio::test]
async fn export_returns_the_user_snapshot() {
    let snapshot = ExportUserData::new(EmptyUserData)
        .execute(ExportUserDataInput {
            owner: UserId::new("user-1").unwrap(),
        })
        .await
        .unwrap();

    assert!(snapshot.dashboard.is_none());
    assert!(snapshot.preferences.is_none());
    assert!(snapshot.themes.is_empty());
    assert!(snapshot.applications.is_empty());
}

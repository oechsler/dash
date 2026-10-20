use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    transfer::{
        application::{models::UserDataSnapshot, ports::UserDataReader},
        application::{
            models::formats::V2Format,
            queries::{ExportUserData, ExportUserDataInput},
            services::ExportService,
        },
    },
    user::domain::UserId,
};

struct EmptyUserData;

#[async_trait]
impl UserDataReader for EmptyUserData {
    async fn load(&self, _owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        Ok(UserDataSnapshot::default())
    }
}

#[tokio::test]
async fn export_emits_signed_v2_transfer_contract() {
    let export = ExportUserData::new(EmptyUserData, ExportService::new(V2Format))
        .execute(ExportUserDataInput {
            owner: UserId::new("user-1").unwrap(),
            username: "sam".into(),
            include_applications: false,
        })
        .await
        .unwrap();

    assert_eq!(export.version, 2);
    assert_eq!(export.username, "sam");
    assert!(export.applications.is_empty());
    assert!(export.verify().unwrap());
}

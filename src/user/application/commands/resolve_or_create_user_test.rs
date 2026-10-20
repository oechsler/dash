use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    user::{
        application::ports::{IdentityWriter, ResolvedUser},
        domain::UserId,
    },
};

use super::{ExternalIdentity, ResolveOrCreateUser};

struct Provisioner {
    received: Arc<Mutex<Option<(String, String)>>>,
    result: ResolvedUser,
}

#[async_trait]
impl IdentityWriter for Provisioner {
    async fn resolve_or_create(
        &self,
        issuer: &str,
        subject: &str,
    ) -> Result<ResolvedUser, ApplicationError> {
        *self.received.lock().unwrap() = Some((issuer.to_owned(), subject.to_owned()));
        Ok(self.result.clone())
    }
}

#[tokio::test]
async fn command_resolves_identity_and_returns_creation_status() {
    let received = Arc::new(Mutex::new(None));
    let provisioner = Provisioner {
        received: received.clone(),
        result: ResolvedUser {
            user_id: UserId::new("internal-1").unwrap(),
            is_new: true,
        },
    };
    let command = ResolveOrCreateUser::new(provisioner);
    let identity = ExternalIdentity {
        issuer: "https://id.example".into(),
        subject: "subject-1".into(),
    };

    let (user_id, is_new) = command.execute(&identity).await.unwrap();

    assert_eq!(user_id.as_str(), "internal-1");
    assert!(is_new);
    assert_eq!(
        received.lock().unwrap().as_ref().unwrap(),
        &("https://id.example".into(), "subject-1".into())
    );
}

use crate::user::domain::UserId;

use crate::{shared::application::ApplicationError, user::application::ports::IdentityWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalIdentity {
    pub issuer: String,
    pub subject: String,
}

/// Command used after successful external authentication. Provisioning is a write-side action.
pub struct ResolveOrCreateUser<R> {
    repository: R,
}

impl<R: IdentityWriter> ResolveOrCreateUser<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        identity: &ExternalIdentity,
    ) -> Result<(UserId, bool), ApplicationError> {
        let resolved = self
            .repository
            .resolve_or_create(&identity.issuer, &identity.subject)
            .await?;
        Ok((resolved.user_id, resolved.is_new))
    }
}

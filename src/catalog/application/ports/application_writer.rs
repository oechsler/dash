use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::catalog::domain::{Application, ApplicationId};

#[async_trait]
pub trait ApplicationWriter: Send + Sync {
    async fn save(&self, application: &Application) -> PortResult<()>;
    async fn delete(&self, id: ApplicationId) -> PortResult<()>;
}

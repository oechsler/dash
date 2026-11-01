use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::catalog::domain::{Application, ApplicationId};

#[async_trait]
pub trait ApplicationReader: Send + Sync {
    async fn list(&self) -> PortResult<Vec<Application>>;
    async fn get(&self, id: ApplicationId) -> PortResult<Option<Application>>;
}

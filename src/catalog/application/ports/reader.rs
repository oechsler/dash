use async_trait::async_trait;

use crate::{
    catalog::domain::{Application, ApplicationId},
    shared::application::ApplicationPortError,
};

#[async_trait]
pub trait ApplicationReader: Send + Sync {
    async fn list(&self) -> Result<Vec<Application>, ApplicationPortError>;
    async fn get(&self, id: ApplicationId) -> Result<Option<Application>, ApplicationPortError>;
}

use async_trait::async_trait;

use crate::{
    catalog::domain::{Application, ApplicationId},
    shared::application::ApplicationPortError,
};

#[async_trait]
pub trait ApplicationWriter: Send + Sync {
    async fn save(&self, application: &Application) -> Result<(), ApplicationPortError>;
    async fn delete(&self, id: ApplicationId) -> Result<(), ApplicationPortError>;
}

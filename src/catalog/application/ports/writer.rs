use async_trait::async_trait;

use crate::{
    catalog::domain::{Application, ApplicationId},
    shared::application::ApplicationError,
};

#[async_trait]
pub trait ApplicationWriter: Send + Sync {
    async fn save(&self, application: &Application) -> Result<(), ApplicationError>;
    async fn delete(&self, id: ApplicationId) -> Result<(), ApplicationError>;
}

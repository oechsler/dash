use async_trait::async_trait;

use crate::{catalog::domain::ApplicationCatalogSettings, shared::application::ApplicationError};

#[async_trait]
pub trait ApplicationCatalogSettingsReader: Send + Sync {
    async fn load(&self) -> Result<Option<ApplicationCatalogSettings>, ApplicationError>;
}

#[async_trait]
pub trait ApplicationCatalogSettingsWriter: Send + Sync {
    async fn save(&self, settings: &ApplicationCatalogSettings) -> Result<(), ApplicationError>;
}

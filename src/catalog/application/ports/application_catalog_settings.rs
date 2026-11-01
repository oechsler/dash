use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::catalog::domain::ApplicationCatalogSettings;

#[async_trait]
pub trait ApplicationCatalogSettingsReader: Send + Sync {
    async fn load(&self) -> PortResult<Option<ApplicationCatalogSettings>>;
}

#[async_trait]
pub trait ApplicationCatalogSettingsWriter: Send + Sync {
    async fn save(&self, settings: &ApplicationCatalogSettings) -> PortResult<()>;
}

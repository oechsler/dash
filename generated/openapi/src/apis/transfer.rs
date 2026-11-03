use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ExportTransferResponse {
    /// Your JSON backup is ready to download.
    Status200_YourJSONBackupIsReadyToDownload {
        body: models::ExportTransfer200Response,
        content_disposition: Option<String>,
    },
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ImportTransferResponse {
    /// Your dashboard has been restored from the backup.
    Status204_YourDashboardHasBeenRestoredFromTheBackup,
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

/// Transfer
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Transfer<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    type Claims;

    /// Download a backup of your dashboard.
    ///
    /// ExportTransfer - GET /api/v1/transfer/export
    async fn export_transfer(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<ExportTransferResponse, E>;

    /// Restore your dashboard from a backup.
    ///
    /// ImportTransfer - POST /api/v1/transfer/import
    async fn import_transfer(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: Multipart,
    ) -> Result<ImportTransferResponse, E>;
}

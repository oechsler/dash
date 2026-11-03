use async_trait::async_trait;
use axum::extract::Multipart;
use axum_extra::extract::CookieJar;
use dash_api::{apis::transfer as api, models};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    shared::application::{ApplicationError, ApplicationResult},
    transfer::presentation::formats::TransferFormat,
};

fn check(c: &HttpClaims, scope: &str) -> ApplicationResult<()> {
    if c.allows(scope) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}

#[async_trait]
impl api::Transfer<ApplicationError> for HttpApi {
    type Claims = HttpClaims;
    async fn export_transfer(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
    ) -> ApplicationResult<api::ExportTransferResponse> {
        check(c, "dash_transfer_read")?;
        let snapshot = crate::transfer::presentation::http::export_transfer_handler::handle(
            self.transfer.clone(),
            c.user_id.clone(),
        )
        .await?;
        let data = crate::transfer::presentation::formats::V2Format.export(
            snapshot,
            &c.user_id,
            c.oidc_identity
                .as_ref()
                .map(|x| x.username.clone())
                .unwrap_or_else(|| c.user_id.as_str().to_owned()),
            c.is_administrator,
        )?;
        let body: models::ExportTransfer200Response = serde_json::from_slice(&data)?;
        Ok(
            api::ExportTransferResponse::Status200_YourJSONBackupIsReadyToDownload {
                body,
                content_disposition: Some("attachment; filename=\"dash-backup.json\"".to_owned()),
            },
        )
    }
    async fn import_transfer(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        mut body: Multipart,
    ) -> ApplicationResult<api::ImportTransferResponse> {
        check(c, "dash_transfer_write")?;
        let mut file = None;
        while let Some(field) = body
            .next_field()
            .await
            .map_err(|e| ApplicationError::infrastructure("read transfer upload", e))?
        {
            if file.is_some() {
                return Err(ApplicationError::Transfer(
                    crate::transfer::application::TransferError::InvalidDocument(
                        "upload must contain a single file".to_owned(),
                    ),
                ));
            }
            file = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| ApplicationError::infrastructure("read transfer file", e))?,
            );
        }
        let bytes = file.ok_or_else(|| {
            ApplicationError::Transfer(
                crate::transfer::application::TransferError::InvalidDocument(
                    "transfer file is missing".to_owned(),
                ),
            )
        })?;
        crate::transfer::presentation::http::import_transfer_handler::handle(
            self.transfer.clone(),
            self.transfer.clone(),
            c.user_id.clone(),
            c.is_administrator,
            &bytes,
        )
        .await?;
        Ok(api::ImportTransferResponse::Status204_YourDashboardHasBeenRestoredFromTheBackup)
    }
}

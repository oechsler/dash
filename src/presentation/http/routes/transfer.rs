use crate::{
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::HttpError,
        middleware::require_scope,
    },
    transfer::presentation::formats::TransferFormat,
};
use axum::{
    Router,
    extract::{Multipart, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};

pub fn router() -> Router<HttpApi> {
    Router::new()
        .route("/transfer/export", get(export_transfer))
        .route("/transfer/import", post(import_transfer))
}
async fn export_transfer(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_transfer_read")?;
    let snapshot = crate::transfer::presentation::http::export_transfer_handler::handle(
        api.transfer,
        claims.user_id.clone(),
    )
    .await?;
    let data = crate::transfer::presentation::formats::V2Format.export(
        snapshot,
        &claims.user_id,
        claims
            .oidc_identity
            .as_ref()
            .map(|i| i.username.clone())
            .unwrap_or_else(|| claims.user_id.as_str().to_owned()),
        claims.is_administrator,
    )?;
    let mut response = (
        StatusCode::OK,
        axum::Json(
            serde_json::from_slice::<dash_api::models::ExportTransfer200Response>(&data)
                .map_err(crate::shared::application::ApplicationError::from)?,
        ),
    )
        .into_response();
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"dash-backup.json\""
            .parse()
            .expect("static disposition"),
    );
    Ok(response)
}
async fn import_transfer(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    mut multipart: Multipart,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_transfer_write")?;
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        crate::shared::application::ApplicationError::infrastructure("read transfer upload", e)
    })? {
        if file.is_some() {
            return Err(crate::shared::application::ApplicationError::BadRequest.into());
        }
        file = Some(field.bytes().await.map_err(|e| {
            crate::shared::application::ApplicationError::infrastructure("read transfer file", e)
        })?);
    }
    let bytes = file.ok_or(crate::shared::application::ApplicationError::BadRequest)?;
    crate::transfer::presentation::http::import_transfer_handler::handle(
        api.transfer.clone(),
        api.transfer,
        claims.user_id,
        claims.is_administrator,
        &bytes,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

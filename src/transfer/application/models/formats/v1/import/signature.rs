use sha2::{Digest, Sha256};

use crate::{
    shared::application::ApplicationError,
    transfer::application::{error::TransferError, models::formats::content_hash::sha256_hex},
};

use super::model::GoExportV1;

pub(super) fn parse(payload: &[u8]) -> Result<GoExportV1, ApplicationError> {
    let mut legacy: GoExportV1 = serde_json::from_slice(payload)?;
    let signature = legacy
        .signature
        .take()
        .ok_or(TransferError::MissingSignature)?;
    let canonical = go_json(serde_json::to_string(&legacy)?);
    let digest: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
    if sha256_hex(&digest) != signature {
        return Err(TransferError::InvalidSignature.into());
    }
    Ok(legacy)
}

fn go_json(json: String) -> String {
    json.replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

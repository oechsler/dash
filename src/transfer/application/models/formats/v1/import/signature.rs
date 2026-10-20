use sha2::{Digest, Sha256};

use crate::{shared::application::ApplicationError, transfer::application::error::TransferError};

use super::super::model::GoExportV1;

pub(super) fn parse(payload: &[u8]) -> Result<GoExportV1, ApplicationError> {
    let mut legacy: GoExportV1 = serde_json::from_slice(payload)?;
    let signature = legacy
        .signature
        .take()
        .ok_or(TransferError::MissingSignature)?;
    let canonical = go_json(serde_json::to_string(&legacy)?);
    let digest: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
    if encode_hex(&digest) != signature {
        return Err(TransferError::InvalidSignature.into());
    }
    Ok(legacy)
}

fn encode_hex(bytes: &[u8; 32]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(DIGITS[(byte >> 4) as usize] as char);
        output.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    output
}

fn go_json(json: String) -> String {
    json.replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}
